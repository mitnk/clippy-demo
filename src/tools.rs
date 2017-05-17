
pub fn rlog(s: &str) {
    // do other stuff ...
    println!("[log] {}", s);
}

macro_rules! log {
    ($fmt:expr) => (
        rlog(format!(concat!($fmt, "\n")).as_str());
    );
    ($fmt:expr, $($arg:tt)*) => (
        rlog(format!(concat!($fmt, "\n"), $($arg)*).as_str());
    );
}
