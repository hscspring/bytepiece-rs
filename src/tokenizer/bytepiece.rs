use std::{fs::File, collections::HashMap, f64::NEG_INFINITY, str};

use unic_normal::StrNormalForm;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use bytes::Bytes;
use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use lazy_static::lazy_static;
use serde_json;
use serde::Deserialize;
use serde_json::Result;


type TokenMap = HashMap<Bytes, usize>;
type IdMap = HashMap<usize, Bytes>;
type Token2ScoreMap = HashMap<Bytes, f64>;


static DEFAULT_MODEL: &str = include_str!("model/bytepiece_80k.model");


pub fn normalize(text: &str) -> Vec<String>  {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*\n+|.+").unwrap();
    }
    let normalized_text = text.nfc().collect::<String>();
    let mut text_list: Vec<String> = Vec::new();
    for mat in RE.find_iter(&normalized_text) {
        let part = mat.as_str();
        text_list.push(part.to_owned());
    }
    text_list
}


pub fn load_model(path: &str) -> String {
    let file = File::open(path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    json.to_string()
}


#[derive(Debug)]
pub struct Tokenizer {
    token_to_ids: TokenMap,
    id_to_tokens: IdMap,
    token_to_score: Token2ScoreMap,
    automaton: Option<AhoCorasick>,
}


#[derive(Debug, Deserialize)]
struct ModelData {
    id: usize,
    _value: String,
    freq: usize,
}


impl Tokenizer {

    pub fn init_empty() -> Self {
        Tokenizer {
            token_to_ids: TokenMap::new(),
            id_to_tokens: IdMap::new(),
            token_to_score: Token2ScoreMap::new(),
            automaton: None,
        }
    }

    pub fn load_from(model_path: &str) -> Self {
        let mut ins = Self::init_empty();
        let model = load_model(model_path);
        ins.build_model(model.as_str()).unwrap();
        ins
    }

    pub fn new() -> Self {
        let mut ins = Self::init_empty();
        ins.build_model(DEFAULT_MODEL).unwrap();
        ins
    }

    fn build_model(& mut self, model_content: &str) -> Result<()> {
        let model: HashMap<String, ModelData> = serde_json::from_str(model_content)?;
        let mut patterns: Vec<Bytes> = Vec::new();
        let mut total_freq: usize = 0;
        let special_tokens = vec!["<pad>", "<bos>", "<eos>"];

        for (i, spe_token) in special_tokens.iter().enumerate() {
            self.token_to_ids.insert(Bytes::from(spe_token.to_owned()), i);
            self.id_to_tokens.insert(i, Bytes::from(spe_token.to_owned()));
        }
        for (key, value) in model {
            let token_u8 = general_purpose::STANDARD.decode(key.as_str()).unwrap();
            let token = Bytes::from(token_u8);
            patterns.push(token.clone());
            self.token_to_ids.insert(token.clone(), value.id);
            self.id_to_tokens.insert(value.id, token.clone());
            self.token_to_score.insert(token.clone(), (value.freq as f64).ln());
            total_freq += value.freq;
        }
        let log_total = (total_freq as f64).ln();
        for (_key, value) in self.token_to_score.iter_mut() {
            *value = *value - log_total as f64;
        }

        self.automaton = Some(
            AhoCorasickBuilder::new()
                .match_kind(MatchKind::Standard)
                .build(patterns)
                .unwrap()
        );

        Ok(())
    }

    fn _tokenize(&self, text_bytes: Bytes) -> Vec<Bytes> {
        let mut tokens = vec![];
        let mut routes: Vec<(f64, usize)> = vec![(0.0, 0)];
        for _char_byte in text_bytes.iter() {
            routes.push((NEG_INFINITY, 0));
        }
        for mat in self.automaton.as_ref().unwrap().find_overlapping_iter(text_bytes.as_ref()) {
            let mat_u8 = text_bytes[mat.start().. mat.end()].to_owned();
            let mat_bytes = Bytes::from(mat_u8);
            let mut score = self.token_to_score[&mat_bytes];
            score += routes[mat.start()].0;
            if score > routes[mat.end()].0 {
                routes[mat.end()] = (score, mat.start());
            }
        }
        let mut end = text_bytes.len();
        while end > 0 {
            let start = routes[end].1;
            let byte_token = text_bytes[start..end].to_owned();
            let token = Bytes::from(byte_token);
            tokens.push(token);
            end = start;
        }
        tokens.reverse();
        tokens
    }
    
    pub fn tokenize(&self, text:  &str) -> Vec<Bytes> {
        let text_list = normalize(text);
        let mut tokens = vec![];
        for p in text_list {
            let part = Bytes::from(p);
            let token_bytes = self._tokenize(part);
            for token_byte in token_bytes {
                tokens.push(token_byte);
            }
        }
        tokens
    }

    pub fn encode(&self, text: &str, add_bos: bool, add_eos: bool) -> Vec<usize> {
        let tokens = self.tokenize(text);
        let mut token_ids = vec![];
        if add_bos {
            token_ids.push(1);
        }
        for token in tokens {
            let token_id = self.token_to_ids[&token];
            token_ids.push(token_id);
        }
        if add_eos {
            token_ids.push(2);
        }
        token_ids
    }

    pub fn decode(&self, token_ids: Vec<usize>) -> String {
        let tokens = token_ids.iter().map(|&x| self.id_to_tokens[&x].clone()).collect::<Vec<Bytes>>();
        let mut text = String::new();
        for token in tokens {
            match str::from_utf8(token.as_ref()) {
                Ok(v) => text.push_str(v),
                Err(_e) => continue,
            }
        }
        text
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model() {
        let tokenizer = Tokenizer::new();
        let text = "今天天气不错";
        let ids = tokenizer.encode(text, false, false);
        assert_eq!(ids, vec![40496, 45268, 39432]);
        let text2 = tokenizer.decode(ids);
        assert_eq!(text2, text);
    }

    #[test]
    fn test_model_loader() {
        let current_dir = std::env::current_dir().unwrap();
        let model_path = current_dir.join("src/tokenizer/model/bytepiece_80k.model");
        let tokenizer = Tokenizer::load_from(model_path.to_str().unwrap());
        let text = "今天天气不错";
        let ids = tokenizer.encode(text, false, false);
        assert_eq!(ids, vec![40496, 45268, 39432]);
    }

    #[test]
    fn test_tokenize() {
        
    }
}