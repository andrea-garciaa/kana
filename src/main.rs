use std::{env};
use kana::KanaSequence;

fn main() {
    let mut args = env::args().skip(1);
    let iteration_count = args.nth(0).unwrap_or(String::from("1")).parse().unwrap_or(1);

    let romaji = args.collect::<Vec<String>>().join(" ");
    let mut kanas = Option::<KanaSequence>::None;

    println!("iteration count: {}", iteration_count);

    let start = std::time::Instant::now();

    for _ in 0..iteration_count {
        kanas = Some(KanaSequence::from_hepburn(&romaji));
    }

    println!("{} ms", start.elapsed().as_nanos() as f32 / 1000000f32);

    if let Some(x) = kanas {
        println!("{}", x.to_hiragana());
        println!("{}", x.to_katakana());
    }

    println!("Byte array representation of romaji input: {:02X?}", romaji.as_bytes());
}