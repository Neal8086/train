

#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        println!("{}:{}:{}", $lvl, $target, $($arg)*);
    });
    ($lvl:expr, $($arg:tt)+) => (log!(target: module_path!(), $lvl, $($arg)+))
}


#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, "ERROR", $($arg)*);
    );
    ($($arg:tt)*) => (
        log!("ERROR", $($arg)*);
    )
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, "WARN", $($arg)*);
    );
    ($($arg:tt)*) => (
        log!("WARN", $($arg)*);
    )
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, "INFO", $($arg)*);
    );
    ($($arg:tt)*) => (
        log!("INFO", $($arg)*);
    )
}