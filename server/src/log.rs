// FIXME: A macro should be used to create this macros,
// but macro creation via macro requires an experimental feature,
// and this project is made to be runnable on stable rust. An
// alternative would be using `build.rs`, but that just adds
// unnecessary complexity.

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", ::colored::Colorize::bright_cyan("info ~>"), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{} {}", ::colored::Colorize::bright_yellow("warn ~>"), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", ::colored::Colorize::bright_red("error ~>"), format!($($arg)*));
    };
}
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("{} {}", ::colored::Colorize::bright_green("success ~>"), format!($($arg)*));
    };
}
