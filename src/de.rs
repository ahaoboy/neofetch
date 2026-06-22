use crate::error::{NeofetchError, Result};
use crate::os::OS;

pub fn get_de(os: OS) -> Result<String> {
    let s = os.to_string();
    if s.contains("Windows 11") {
        return Ok(String::from("Fluent"));
    }

    if s.contains("Windows 10") {
        return Ok(String::from("Fluent"));
    }
    if s.contains("Windows 8") {
        return Ok(String::from("Metro"));
    }
    if s.contains("Windows") {
        return Ok(String::from("Aero"));
    }
    if s.contains("Linux") && std::env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
        return Ok(String::from("GNOME"));
    }

    Err(NeofetchError::data_unavailable(
        "Desktop environment not detected",
    ))
}
