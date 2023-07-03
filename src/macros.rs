#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => (
        unsafe {
            if crate::DEBUG_ENABLED {
                println!("{}", format_args!($($args)*))
            }
        }
    );
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        println!("WARN: {}", format_args!($($args)*))
    };
}
