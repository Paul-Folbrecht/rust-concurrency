use std::collections::HashMap;
use std::{sync::mpsc, thread::spawn};
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
    let (sender, receiver) = mpsc::channel();
    let num_cores = 4;
    let chunks = lines.chunks(lines.len() / num_cores);

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let sender = mpsc::Sender::clone(&sender);
            let owned = chunk.to_owned();
            spawn(move || {
                let map = count_words(owned);
                sender.send(map).unwrap();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
    drop(sender);

    let mut word_counts: Vec<_> = collect(receiver).into_iter().collect();
    word_counts.sort_by(|x, y| y.1.cmp(&x.1));
    for (key, value) in word_counts.into_iter().take(10) {
        println!("{}: {}", key, value);
    }
}

fn collect(receiver: mpsc::Receiver<HashMap<String, u32>>) -> HashMap<String, u32> {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    for map in receiver {
        for (word, count) in map {
            let total = word_counts.entry(word).or_insert(0);
            *total += count;
        }
    }
    word_counts
}
