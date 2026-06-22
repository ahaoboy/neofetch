use crate::error::{NeofetchError, Result};
use public_ip_address::perform_lookup;
use std::net::UdpSocket;

pub fn get_local_ip() -> Result<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .map_err(|e| NeofetchError::system_call(format!("Failed to bind UDP socket: {}", e)))?;
    socket
        .connect("8.8.8.8:80")
        .map_err(|e| NeofetchError::system_call(format!("Failed to connect UDP socket: {}", e)))?;
    let local_addr = socket
        .local_addr()
        .map_err(|e| NeofetchError::system_call(format!("Failed to get local address: {}", e)))?;
    Ok(local_addr.ip().to_string())
}

pub async fn get_ip() -> Result<String> {
    let response = perform_lookup(None)
        .await
        .map_err(|e| NeofetchError::system_call(format!("Failed to perform public IP lookup: {}", e)))?;

    let ip = response.ip;
    let s = match (response.country_code, response.city) {
        (Some(country), Some(city)) => format!("{} ({}-{})", ip, country, city),
        (Some(country), None) => format!("{} ({})", ip, country),
        _ => format!("{}", ip),
    };
    Ok(s)
}
