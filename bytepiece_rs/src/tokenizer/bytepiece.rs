use std::{fs::File, collections::HashMap, f64::NEG_INFINITY, str};

use rand::Rng;
use unic_normal::StrNormalForm;
use regex::Regex;
use lazy_static::lazy_static;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use base64::{Engine as _, engine::general_purpose};
use serde_json;
use serde::Deserialize;
use serde_json::Result;
use rayon::prelude::*;



type TokenMap = HashMap<Vec<u8>, usize>;
type IdMap = HashMap<usize, Vec<u8>>;
type Token2ScoreMap = HashMap<Vec<u8>, f64>;


static DEFAULT_MODEL: &str = include_str!("model/bytepiece_80k.model");


fn load_model(path: &str) -> String {
    let file = File::open(path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    json.to_string()
}


pub fn chunk(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*\n+|.+").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}


#[inline(always)]
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}


#[inline(always)]
pub fn _sigmoid(x: f64) -> f64 {
    if x >= 0.0 {
        1.0 / (1.0 + (-x).exp())
    } else {
        1.0 - 1.0 / (1.0 + x.exp())
    }
}


#[inline(always)]
pub fn logsumexp(x: f64, y: f64) -> f64 {
    if x == NEG_INFINITY {
        y
    } else if y == NEG_INFINITY {
        x
    } else if x > y {
        x + (1.0 + (y - x).exp()).ln()
    } else {
        y + (1.0 + (x - y).exp()).ln()
    }
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


impl<'a> Tokenizer {

    fn init_empty() -> Self {
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
        ins.build_model(&model).unwrap();
        ins
    }

    pub fn new() -> Self {
        let mut ins = Self::init_empty();
        ins.build_model(DEFAULT_MODEL).unwrap();
        ins
    }

    fn build_model(& mut self, model_content: &str) -> Result<()> {
        let model: HashMap<String, ModelData> = serde_json::from_str(model_content)?;
        let mut patterns: Vec<Vec<u8>> = Vec::new();
        let mut total_freq: usize = 0;
        let special_tokens = ["<pad>", "<bos>", "<eos>"];

        for (i, spe_token) in special_tokens.iter().enumerate() {
            self.token_to_ids.insert(spe_token.as_bytes().to_vec(), i);
            self.id_to_tokens.insert(i, spe_token.as_bytes().to_vec());
        }
        for (key, value) in model {
            let token_u8 = general_purpose::STANDARD.decode(key.as_str()).unwrap();
            patterns.push(token_u8.clone());
            self.token_to_ids.insert(token_u8.clone(), value.id);
            self.id_to_tokens.insert(value.id, token_u8.clone());
            self.token_to_score.insert(token_u8.clone(), (value.freq as f64).ln());
            total_freq += value.freq;
        }
        let log_total = (total_freq as f64).ln();
        for value in self.token_to_score.values_mut() {
            *value -= log_total;
        }

        patterns.sort();
        self.automaton = Some(
            AhoCorasickBuilder::new()
                .match_kind(MatchKind::Standard)
                .build(&patterns)
                .unwrap()
        );

        Ok(())
    }

    fn tokenize_bytes(&'a self, text_bytes: &'a [u8], alpha: f64) -> Vec<usize> {
        if alpha < 0.0 {
            self.viterbi_decode(text_bytes)
        } else {
            self.viterbi_sample(text_bytes, alpha)
        }
    }

    fn viterbi_sample(&'a self, text_bytes: &'a [u8], alpha: f64) -> Vec<usize> {
        let len = text_bytes.len() + 1;
        let mut scores: Vec<f64> = vec![NEG_INFINITY; len];
        let mut logsumexp_scores: Vec<f64> = vec![NEG_INFINITY; len];
        scores[0] = 0.0;
        logsumexp_scores[0] = 0.0;
        let automaton = self.automaton.as_ref().unwrap(); 
        let mut routes: Vec<usize> = (0..len).collect();
        for mat in automaton.find_overlapping_iter(text_bytes.as_ref()) {
            let end = mat.end();
            let start = mat.start();
            let mat_u8 = &text_bytes[start..end];
            let score = self.token_to_score[mat_u8] * alpha + scores[start];
            let lse_score = logsumexp(scores[end], score);
            scores[end] = lse_score;
            if random() < (score - lse_score).exp() {
                scores[end] = score;
                routes[end] = start;
            }
        }
        let mut token_ids = vec![];
        let mut end = len - 1;
        while end > 0 {
            let start = routes[end];
            let byte_token = &text_bytes[start..end];
            token_ids.push(self.token_to_ids[byte_token]);
            end = start;
        }
        token_ids.reverse();
        token_ids
    }

    fn viterbi_decode(&'a self, text_bytes: &'a [u8]) -> Vec<usize> {
        let len = text_bytes.len() + 1;
        let mut scores: Vec<f64> = vec![NEG_INFINITY; len];
        scores[0] = 0.0;
        let automaton = self.automaton.as_ref().unwrap(); 
        let mut routes: Vec<usize> = (0..len).collect();
        for mat in automaton.find_overlapping_iter(text_bytes.as_ref()) {
            let end = mat.end();
            let start = mat.start();
            let mat_u8 = &text_bytes[start..end];
            let score = self.token_to_score[mat_u8] + scores[start];
            if score > scores[end] {
                scores[end] = score;
                routes[end] = start;
            }
        }
        let mut token_ids = vec![];
        let mut end = len - 1;
        while end > 0 {
            let start = routes[end];
            let byte_token = &text_bytes[start..end];
            token_ids.push(self.token_to_ids[byte_token]);
            end = start;
        }
        token_ids.reverse();
        token_ids
    }

    pub fn tokenize(&self, text: &str, alpha: f64, norm: bool) -> Vec<usize> {
        match norm {
            true => {
                let norm_text = text.nfc().collect::<String>();
                self._tokenize(&norm_text, alpha)
            }
            false => self._tokenize(text, alpha)
        }
    }

    fn _tokenize(&'a self, text: &str, alpha: f64) -> Vec<usize> {
        let text_list = chunk(text);
        let mut token_ids = vec![];
        if text_list.len() > 1 && text.len() > 2048 {
            let x = text_list.into_par_iter().map(|p| {
                let p_ids = self.tokenize_bytes(p.as_bytes(), alpha);
                p_ids
            });
            x.collect_into_vec(&mut token_ids);
        } else {
            for p in text_list {
                let p_ids = self.tokenize_bytes(p.as_bytes(), alpha);
                token_ids.push(p_ids);
            }
        }
        token_ids.concat()
    }

    pub fn encode(
        &self, text: &str, add_bos: bool, add_eos: bool, alpha: f64, norm: bool,
    ) -> Vec<usize> {
        let mut token_ids = self.tokenize(text, alpha, norm);
        if add_bos {
            token_ids.insert(0, 1);
        }
        if add_eos {
            token_ids.push(2);
        }
        token_ids
    }

    pub fn decode(&self, token_ids: Vec<usize>) -> String {
        let tokens: Vec<Vec<u8>> = token_ids
            .iter()
            .map(|&x| self.id_to_tokens[&x].clone())
            .collect();
        let mut text = String::new();
        for token in tokens {
            match str::from_utf8(&token) {
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
        let ids = tokenizer.encode(
            text, false, false, -1.0, false
        );
        assert_eq!(ids, vec![40496, 45268, 39432]);
        let text2 = tokenizer.decode(ids);
        assert_eq!(text2, text);
        let text = "";
        let ids = tokenizer.encode(
            text, false, false, -1.0, false
        );
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_model_loader() {
        let current_dir = std::env::current_dir().unwrap();
        let model_path = current_dir.join("src/tokenizer/model/bytepiece_80k.model");
        let tokenizer = Tokenizer::load_from(model_path.to_str().unwrap());
        let text = "今天天气不错";
        let ids = tokenizer.encode(
            text, false, false, -1.0, false
        );
        assert_eq!(ids, vec![40496, 45268, 39432]);
    }

    #[test]
    fn test_chunk() {
        let text = "今天天气不错";
        let text_list = chunk(text);
        assert_eq!(text_list, vec!["今天天气不错"]);
        let text2 = "今天天气不错\n";
        let text_list2 = chunk(text2);
        assert_eq!(text_list2, vec!["今天天气不错\n"]);
        let text3 = "今天天气不错\n今天天气不错";
        let text_list3 = chunk(text3);
        assert_eq!(text_list3, vec!["今天天气不错\n", "今天天气不错"]);
    }

    #[test]
    fn test_tokenize() {
        let tokenizer = Tokenizer::new();
        let text = "今天天气不错";
        let tokens = tokenizer.tokenize(text, 0.0, false);
        assert_eq!(tokens.len() >= 3, true);
        let tokens = tokenizer.tokenize(text, -1.0, false);
        assert_eq!(tokens.len(), 3);
        let tokens = tokenizer.tokenize(text, 1.0, false);
        assert_eq!(tokens.len() == 3, true);
        let tokens = tokenizer.tokenize(text, 3.0, false);
        assert_eq!(tokens.len() == 3, true);
        let tokens = tokenizer.tokenize(text, 0.1, false);
        assert_eq!(tokens.len() >= 3, true);
    }

    macro_rules! sigmoid_test {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let res = expected - _sigmoid(input);
                assert_eq!(res.abs() < 1e-2, true);
            }
        )*}
    }

    sigmoid_test! {
        sigmoid_0: (0.0, 0.5),
        sigmoid_1: (1.0, 0.731),
        sigmoid_2: (-1.0, 0.269),
    }

    macro_rules! logsumexp_test {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (x, y, expected) = $value;
                let res = expected - logsumexp(x, y);
                assert_eq!(res.abs() < 1e-2, true);
            }
        )*}
    }

    logsumexp_test! {
        logsumexp_0: (0.0, 0.0, 0.693),
        logsumexp_1: (2.0, 1.0, 2.313),
        logsumexp_2: (1.0, 2.0, 2.313),
        logsumexp_3: (-1.0, -3.0, -0.873),
        logsumexp_4: (-3.0, -1.0, -0.873),
        logsumexp_5: (NEG_INFINITY, 1.0, 1.0),
        logsumexp_6: (1.0, NEG_INFINITY, 1.0),
    }

    #[test]
    fn test_random() {
        loop {
            let res = random();
            assert_eq!(res >= 0.0 && res < 1.0, true);
            if res > 0.2 {
                break;
            }
        }
    }
}