use std::time::Duration;

use colored::{Colorize, ColoredString, Color};

pub fn format_duration(duration: &Duration) -> ColoredString {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let mins = (duration.as_secs_f32() / 60f32) as u32;
    let secs = duration.as_secs() % 60;
    let micros = duration.subsec_micros();
    
    format!("[{mins:02}:{secs:02}.{micros:06}]")
        .color(speed_color(duration)).bold()
}

fn speed_color(duration: &Duration) -> Color {
    if duration.as_secs() >= 10 { Color::BrightRed } // > 10s
    else if duration.as_secs() >= 1 { Color::TrueColor { r: 255, g: 153, b: 23 } } // 1-10s
    else if duration.as_millis() >= 100 { Color::BrightYellow } // 100-1000ms
    else if duration.as_millis() >= 1  { Color::BrightBlue } // 1-100ms
    else { Color::BrightGreen } // < 1ms
}