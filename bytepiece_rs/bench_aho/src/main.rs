use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use unic_normal::StrNormalForm;
use bytes::Bytes;
use serde::Deserialize;
use aho_corasick::AhoCorasick;
use base64::{Engine as _, engine::general_purpose};



#[derive(Debug, Deserialize)]
struct ModelData {
    _id: usize,
    _value: String,
    _freq: usize,
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let root = current_dir.parent().unwrap();

    let model_path = root.join("src/tokenizer/model/bytepiece_80k.model");
    let file = File::open(model_path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let model: HashMap<String, ModelData> = serde_json::from_str(json.to_string().as_str()).unwrap();
    let mut patterns: Vec<Bytes> = Vec::new();
    for (key, _value) in model {
        let token_u8 = general_purpose::STANDARD.decode(key.as_str()).unwrap();
        let token = Bytes::from(token_u8);
        patterns.push(token.clone());
    }
    patterns.sort();
    let aho = AhoCorasick::new(&patterns).unwrap();
    let file_path = current_dir.join("data/鲁迅全集.txt");
    let mut file = File::open(file_path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let normalized_text = text.nfc().collect::<String>();
    for total in [100, 1000, 10000, 100000] {
        let test_text = normalized_text.chars().take(total).collect::<String>();
        let byte_text = Bytes::from(test_text);
        let mut count = 0;
        let start_time = Instant::now();
        for _mat in aho.find_overlapping_iter(byte_text.as_ref()) {
            count += 1;
        }
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Count: {}, Elapsed time: {:?}", total, elapsed_time);
    }
}
