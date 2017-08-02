
macro_rules! verbose {
    ($v:expr) => {
        if $v {
            eprintln!("     {}", "info:".blue().bold());
        }
    };
    ($v:expr, $fmt:expr) => {
        if $v {
            eprintln!(concat!("     {} ", $fmt), "info:".blue().bold());
        }
    };
    ($v:expr, $fmt:expr, $($arg:tt)*) => {
        if $v {
            eprintln!(concat!("     {} ", $fmt), "info:".blue().bold(), $($arg)*);
        }
    };
}
