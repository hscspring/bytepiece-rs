use std::time::Instant;

use bytepiece_rs::Tokenizer;
use bytepiece_rs::read_to_string;

fn main() {
    let tokenizer = Tokenizer::new();
    let text = "今天天气不错";
    let tokens = tokenizer.tokenize(text, 0.1);
    println!("{:?}", tokens);

    let start_time = Instant::now();
    let text = read_to_string("bench_aho/data/鲁迅全集.txt");
    let _ids = tokenizer.encode(&text, false, false, 0.0);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Text len: {}, Elapsed time: {:?}", text.len(), elapsed_time);
}
