use std::time::Instant;

use bytepiece_rs::Tokenizer;
use bytepiece_rs::read_to_string;


#[test]
fn test_tokenizer() {
    let tokenizer = Tokenizer::new();
    let start_time = Instant::now();
    let text = read_to_string("bench_aho/data/鲁迅全集.txt");
    let test_text = text.chars().take(1000).collect::<String>();
    let _ids = tokenizer.encode(
        &test_text, false, false, 0.0, false
    );
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Text len: {}, Elapsed time: {:?}", text.len(), elapsed_time);
}
