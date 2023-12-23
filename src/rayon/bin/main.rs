use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
#[path = "../../util.rs"]
mod util;
use util::*;

fn main() {
    time("run", || run());
}

fn run() {
    let lines: Vec<&str> = include_str!("../../resources/100MB.txt")
        .split("\n")
        .collect();

    let maps: Vec<HashMap<String, u32>> = lines
        .into_par_iter()
        .map(|line| count_words(vec![line]))
        .collect();

    let mut word_counts: HashMap<String, u32> = HashMap::new();
    for map in maps {
        for (word, count) in map {
            let total = word_counts.entry(word).or_insert(0);
            *total += count;
        }
    }

    let mut count_vec: Vec<_> = word_counts.into_iter().collect();
    count_vec.sort_by(|x, y| y.1.cmp(&x.1));
    for (key, value) in count_vec.into_iter().take(10) {
        println!("{}: {}", key, value);
    }
}
