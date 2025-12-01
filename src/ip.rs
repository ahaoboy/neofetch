use public_ip_address::perform_lookup;
use std::net::UdpSocket;

pub fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_addr = socket.local_addr().ok()?;
    Some(local_addr.ip().to_string())
}

pub async fn get_ip() -> Option<String> {
    if let Ok(response) = perform_lookup(None).await {
        let ip = response.ip;
        let s = match (response.country_code, response.city) {
            (Some(country), Some(city)) => format!("{} ({}-{})", ip, country, city),
            (Some(country), None) => format!("{} ({})", ip, country),
            _ => format!("{}", ip),
        };
        return Some(s);
    }
    None
}
