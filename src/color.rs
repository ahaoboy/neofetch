pub static BLACK: &str = "\x1b[30m";
pub static RED: &str = "\x1b[31m";
pub static GREEN: &str = "\x1b[32m";
pub static YELLOW: &str = "\x1b[33m";
pub static BLUE: &str = "\x1b[34m";
pub static MAGENTA: &str = "\x1b[35m";
pub static CYAN: &str = "\x1b[36m";
pub static WHITE: &str = "\x1b[37m";

pub static BRIGHT_BLACK: &str = "\x1b[90m";
pub static BRIGHT_RED: &str = "\x1b[91m";
pub static BRIGHT_GREEN: &str = "\x1b[92m";
pub static BRIGHT_YELLOW: &str = "\x1b[93m";
pub static BRIGHT_BLUE: &str = "\x1b[94m";
pub static BRIGHT_MAGENTA: &str = "\x1b[95m";
pub static BRIGHT_CYAN: &str = "\x1b[96m";
pub static BRIGHT_WHITE: &str = "\x1b[97m";

pub static RESET: &str = "\x1b[0m";
pub static BOLD: &str = "\x1b[1m";

pub static BLACK_BG: &str = "\x1b[40m";
pub static RED_BG: &str = "\x1b[41m";
pub static GREEN_BG: &str = "\x1b[42m";
pub static YELLOW_BG: &str = "\x1b[43m";
pub static BLUE_BG: &str = "\x1b[44m";
pub static MAGENTA_BG: &str = "\x1b[45m";
pub static CYAN_BG: &str = "\x1b[46m";
pub static WHITE_BG: &str = "\x1b[47m";

pub static BRIGHT_BLACK_BG: &str = "\x1b[100m";
pub static BRIGHT_RED_BG: &str = "\x1b[101m";
pub static BRIGHT_GREEN_BG: &str = "\x1b[102m";
pub static BRIGHT_YELLOW_BG: &str = "\x1b[103m";
pub static BRIGHT_BLUE_BG: &str = "\x1b[104m";
pub static BRIGHT_MAGENTA_BG: &str = "\x1b[105m";
pub static BRIGHT_CYAN_BG: &str = "\x1b[106m";
pub static BRIGHT_WHITE_BG: &str = "\x1b[107m";

pub static COLORS: [&str; 16] = [
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    BRIGHT_BLACK,
    BRIGHT_RED,
    BRIGHT_GREEN,
    BRIGHT_YELLOW,
    BRIGHT_BLUE,
    BRIGHT_MAGENTA,
    BRIGHT_CYAN,
    BRIGHT_WHITE,
];

pub fn cursor_up(n: usize) -> String {
    format!("\x1B[{n}A")
}
pub fn cursor_down(n: usize) -> String {
    format!("\x1B[{n}B")
}
pub fn cursor_forward(n: usize) -> String {
    format!("\x1B[{n}C")
}
pub fn cursor_backward(n: usize) -> String {
    format!("\x1B[{n}D")
}
