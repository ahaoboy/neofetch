use crate::os::get_os;

pub fn get_de() -> Option<String> {
    if let Some(os) = get_os() {
        let s = os.to_string();
        if s.contains("Windows 11") {
            return String::from("Fluent").into();
        }

        if s.contains("Windows 10") {
            return String::from("Fluent").into();
        }
        if s.contains("Windows 8") {
            return String::from("Metro").into();
        }
        if s.contains("Windows") {
            return String::from("Aero").into();
        }
        if s.contains("Linux") && std::env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
            return String::from("GNOME").into();
        }

        return None;
    }
    None
}
