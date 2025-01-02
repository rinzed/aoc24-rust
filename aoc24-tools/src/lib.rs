use num_format::{Locale, ToFormattedString};
use std::fs::read_to_string;
use std::sync::Mutex;
use std::time::Duration;
use colored::Colorize;

lazy_static::lazy_static! {
    pub static ref MEASUREMENTS: Mutex<Vec<(String, Duration)>> = Mutex::new(Vec::new());
}

#[macro_export]
macro_rules! init_measurements {
    () => {{
        // Ensure lazy_static is initialized
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {init_measurements_once()});
    }};
}

pub fn init_measurements_once() {
    lazy_static::initialize(&MEASUREMENTS);
}

#[macro_export]
macro_rules! measure {
    ($block:block, $name:expr) => {{
        let start = std::time::Instant::now();
        let result = { $block };
        let duration = start.elapsed();

        push_measurement($name.to_string(), duration);

        result
    }};
}

#[macro_export]
macro_rules! measure_total {
    ($block:block) => {{
        measure!($block, "")
    }};
}

pub fn push_measurement(name: String, duration: Duration) {
    MEASUREMENTS.lock().unwrap().push((name, duration));
}

#[inline]
pub fn print_header(day: u8, title: &str) {
    let ast = "*".truecolor(0xFF, 0xFF, 0x66).bold();
    let tripledash = "---".white();
    println!();
    println!(
        " {ast}{ast} {} {ast}{ast}",
        "Advent of Code 2024".truecolor(0x00, 0xCC, 0x00).bold()
    );
    println!("{}", format!("{tripledash} Day {:02}: {:11} {tripledash}", day, title).truecolor(0xFF, 0xFF, 0xFF).bold());
    println!();
}

#[inline]
pub fn print_summary(day: u8) {
    let time = get_default_timing();
    let ns = time.as_nanos().to_formatted_string(&Locale::en);
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {day} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
    print_named_durations();
}

fn get_default_timing() -> Duration {
    MEASUREMENTS
        .lock()
        .unwrap()
        .iter()
        .filter(|(k, _)| k == "")
        .map(|(_, v)| *v)
        .next()
        .unwrap_or_default()
}

#[inline]
fn print_named_durations() {
    let measurements = MEASUREMENTS.lock().unwrap();
    if measurements.len() <= 1 {
        return;
    }

    println!(
        "\n| {:10} | {:^10} | {:>15} |",
        "Part", "Duration", "Nanoseconds"
    );
    let mut sum = Duration::default();
    for (name, duration) in measurements.iter().filter(|(k, _)| *k != "") {
        let nanos = duration.as_nanos().to_formatted_string(&Locale::en);
        println!("| {name:10} | {duration:^10?} | {nanos:>12} ns |");
        sum += *duration;
    }
    let nanos = sum.as_nanos().to_formatted_string(&Locale::en);
    println!("| {:10} | {sum:^10?} | {nanos:>12} ns |", "total");
}
