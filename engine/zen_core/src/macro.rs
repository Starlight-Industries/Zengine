#[macro_export]
macro_rules! printinfo {
    ($($arg:tt)*) => {{
        use chrono::Local;
        use colored::Colorize;
        let current_time = Local::now().format("[%H:%M:%S.%3f/INFO]:").to_string();
        println!("{} {}", current_time.bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! printerr {
    ($error:expr) => {{
        use chrono::Local;
        use colored::Colorize;
        let current_time = Local::now().format("[%H:%M:%S.%3f/ERROR]:").to_string();
        eprintln!("{} {}", current_time.red().bold(), $error.to_string().red());
    }};
    ($($arg:tt)*) => {{
        use chrono::Local;
        use colored::Colorize;
        let current_time = Local::now().format("[%H:%M:%S.%3f/ERROR]:").to_string();
        eprintln!("{} {}", current_time.red().bold(), format_args!($($arg)*).to_string().red());
    }};
}

#[macro_export]
macro_rules! printwarn {
    ($($arg:tt)*) => {{
        use chrono::Local;
        use colored::Colorize;
        let current_time = Local::now().format("[%H:%M:%S.%3f/WARN]:").to_string();
        eprintln!("{} {}", current_time.yellow().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! throw {
    ($error:expr) => {{
        use chrono::Local;
        use colored::Colorize;
        use std::io::Write;

        let error = $error;
        let current_time = Local::now().format("[%H:%M:%S.%3f/EXCEPT]:").to_string();
        let error_message = format!("{}", error);
        let orange = colored::CustomColor {
            r: 255,
            g: 156,
            b: 0,
        };
        
        // Ensure the entire error message is written at once
        let mut stderr = std::io::stderr().lock();
        writeln!(
            stderr,
            "{} {} {}",
            current_time.custom_color(orange).bold(),
            "An error occurred but is still recoverable:".custom_color(orange),
            error_message.custom_color(orange)
        ).expect("Failed to write to stderr");
        
        let backtrace = std::backtrace::Backtrace::capture();
        writeln!(stderr, "└─{}", "Stack trace:".custom_color(orange).bold()).expect("Failed to write to stderr");
        writeln!(stderr, "  └─{}", format!("{:#?}", backtrace).custom_color(orange)).expect("Failed to write to stderr");
        
        stderr.flush().expect("Failed to flush stderr");

        Err(error)
    }};
}
