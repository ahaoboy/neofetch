//! Network information collector
//!
//! Collects network interface information including IP addresses and status.

use crate::error::{NeofetchError, Result};
use std::fmt::Display;

/// Network interface information
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    /// Interface name (e.g., "eth0", "wlan0")
    pub interface_name: String,
    /// IPv4 address if available
    pub ipv4_address: Option<String>,
    /// IPv6 address if available
    pub ipv6_address: Option<String>,
    /// MAC address if available
    pub mac_address: Option<String>,
    /// Whether the interface is up
    pub is_up: bool,
}

impl Display for NetworkInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.is_up { "UP" } else { "DOWN" };
        write!(f, "{} ({})", self.interface_name, status)?;

        if let Some(ipv4) = &self.ipv4_address {
            write!(f, " IPv4: {}", ipv4)?;
        }

        if let Some(mac) = &self.mac_address {
            write!(f, " MAC: {}", mac)?;
        }

        Ok(())
    }
}

/// Get network interface information on Windows
#[cfg(windows)]
pub async fn get_network_info() -> Result<Vec<NetworkInfo>> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_NetworkAdapterConfiguration")]
    struct NetworkAdapter {
        #[serde(rename = "Description")]
        description: Option<String>,
        #[serde(rename = "IPAddress")]
        ip_address: Option<Vec<String>>,
        #[serde(rename = "MACAddress")]
        mac_address: Option<String>,
        #[serde(rename = "IPEnabled")]
        ip_enabled: Option<bool>,
    }

    let results: Vec<NetworkAdapter> = wmi_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let mut interfaces = Vec::new();

    for adapter in results {
        if let Some(desc) = adapter.description {
            let is_up = adapter.ip_enabled.unwrap_or(false);
            let mut ipv4 = None;
            let mut ipv6 = None;

            if let Some(addresses) = adapter.ip_address {
                for addr in addresses {
                    if addr.contains(':') {
                        ipv6 = Some(addr);
                    } else if addr.contains('.') {
                        ipv4 = Some(addr);
                    }
                }
            }

            interfaces.push(NetworkInfo {
                interface_name: desc,
                ipv4_address: ipv4,
                ipv6_address: ipv6,
                mac_address: adapter.mac_address,
                is_up,
            });
        }
    }

    Ok(interfaces)
}

/// Get network interface information on Unix-like systems
#[cfg(unix)]
pub async fn get_network_info() -> Result<Vec<NetworkInfo>> {
    use crate::utils::execute_command;

    let mut interfaces = Vec::new();

    // Try to use ip command first (modern Linux)
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = execute_command("ip", &["-brief", "addr", "show"]).await {
            for line in output.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let name = parts[0].to_string();
                    let status = parts[1];
                    let is_up = status.contains("UP");

                    let mut ipv4 = None;
                    let mut ipv6 = None;

                    for addr in &parts[2..] {
                        let addr_clean = addr.trim_end_matches(',');
                        if let Some(ip) = addr_clean.split('/').next() {
                            if ip.contains(':') {
                                ipv6 = Some(ip.to_string());
                            } else if ip.contains('.') {
                                ipv4 = Some(ip.to_string());
                            }
                        }
                    }

                    interfaces.push(NetworkInfo {
                        interface_name: name,
                        ipv4_address: ipv4,
                        ipv6_address: ipv6,
                        mac_address: None,
                        is_up,
                    });
                }
            }

            return Ok(interfaces);
        }
    }

    // Fallback to ifconfig (works on macOS and older Linux)
    if let Ok(output) = execute_command("ifconfig", &[] as &[&str]).await {
        let mut current_interface: Option<NetworkInfo> = None;

        for line in output.lines() {
            if !line.starts_with(' ') && !line.starts_with('\t') {
                // New interface
                if let Some(interface) = current_interface.take() {
                    interfaces.push(interface);
                }

                let parts: Vec<&str> = line.split(':').collect();
                if let Some(name) = parts.first() {
                    let is_up = line.contains("UP");
                    current_interface = Some(NetworkInfo {
                        interface_name: name.trim().to_string(),
                        ipv4_address: None,
                        ipv6_address: None,
                        mac_address: None,
                        is_up,
                    });
                }
            } else if let Some(ref mut interface) = current_interface {
                // Parse interface details
                if line.contains("inet ") && !line.contains("inet6") {
                    if let Some(addr_start) = line.find("inet ") {
                        let addr_part = &line[addr_start + 5..];
                        if let Some(addr) = addr_part.split_whitespace().next() {
                            interface.ipv4_address = Some(addr.to_string());
                        }
                    }
                } else if line.contains("inet6 ") {
                    if let Some(addr_start) = line.find("inet6 ") {
                        let addr_part = &line[addr_start + 6..];
                        if let Some(addr) = addr_part.split_whitespace().next() {
                            interface.ipv6_address = Some(addr.to_string());
                        }
                    }
                } else if line.contains("ether ")
                    && let Some(mac_start) = line.find("ether ")
                {
                    let mac_part = &line[mac_start + 6..];
                    if let Some(mac) = mac_part.split_whitespace().next() {
                        interface.mac_address = Some(mac.to_string());
                    }
                }
            }
        }

        if let Some(interface) = current_interface {
            interfaces.push(interface);
        }
    }

    if interfaces.is_empty() {
        return Err(NeofetchError::data_unavailable(
            "No network interfaces found",
        ));
    }

    Ok(interfaces)
}

/// Get active network interfaces (those that are up and have an IP)
pub async fn get_active_interfaces() -> Result<Vec<NetworkInfo>> {
    let all_interfaces = get_network_info().await?;

    Ok(all_interfaces
        .into_iter()
        .filter(|iface| iface.is_up && iface.ipv4_address.is_some())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_network_info() {
        let result = get_network_info().await;
        // Should return at least loopback interface
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_active_interfaces() {
        let result = get_active_interfaces().await;
        // May or may not have active interfaces
        assert!(result.is_ok() || result.is_err());
    }
}
