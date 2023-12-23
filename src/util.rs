use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::time::Instant;

pub fn time<T>(label: &str, f: impl FnOnce() -> T) -> T {
    let now = Instant::now();
    let result = f();
    let elapsed = now.elapsed().as_millis();
    println!(
        "Time for {}: {}ms",
        label,
        elapsed.to_formatted_string(&Locale::en)
    );
    result
}

pub fn count_words(lines: Vec<&str>) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for line in lines {
        for word in line.split_ascii_whitespace() {
            let count = map.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }
    map
}
