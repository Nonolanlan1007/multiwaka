enum Color {
    Gray,
    Purple,
    Red,
}

pub struct Logger {}

impl Logger {
    fn colorize(text: &str, color: Color) -> String {
        let start_char = match color {
            Color::Gray => "\x1B[90m",
            Color::Purple => "\x1B[35m",
            Color::Red => "\x1B[31m",
        };

        start_char.to_string() + text + "\x1B[0m"
    }

    fn get_date() -> String {
        let date = chrono::Local::now();

        date.format("%d/%m/%Y %H:%M:%S").to_string()
    }

    fn log(text: &str, color: Option<Color>) {
        println!(
            "{} - {}",
            Self::colorize(&Logger::get_date(), Color::Gray).as_str(),
            match color {
                Some(color) => Self::colorize(&text, color),
                None => text.to_string(),
            }
        );
    }

    pub fn info(text: &str) {
        Self::log(text, None);
    }

    pub fn highlight(text: &str) {
        Self::log(text, Some(Color::Purple));
    }

    pub fn error(text: &str) {
        Self::log(text, Some(Color::Red));
    }
}
