use std::fs::File;
use std::io::Read;
use std::time::Instant;


use bytepiece_rs::Tokenizer;


fn main() {
    let text = "今天天气不错";
    let current_dir = std::env::current_dir().unwrap();

    let model_path = current_dir.join("src/tokenizer/model/bytepiece_80k.model");
    let tokenizer1 = Tokenizer::load_from(model_path.to_str().unwrap());
    let tokenizer2 = Tokenizer::new();
    let ids = tokenizer1.encode(text, false, false);
    println!("{:?}", ids);
    let text2 = tokenizer2.decode(ids);
    println!("{:?}", text2);

    
    let file_path = current_dir.join("bench/data/鲁迅全集.txt");
    let mut file = File::open(file_path.to_str().unwrap()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let start_time = Instant::now();
    let _ids = tokenizer1.encode(&content, false, false);
    let end_time = Instant::now();
    let elapsed_time = (end_time - start_time) * 1000;
    println!("Elapsed time: {:?}", elapsed_time);
}
