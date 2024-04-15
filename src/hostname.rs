use crate::share::exec;

pub fn get_hostname() -> Option<String> {
    let s = exec("hostname", [])?;
    s.into()
}

