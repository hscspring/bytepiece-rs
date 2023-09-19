use pyo3::prelude::*;


#[pyclass(subclass)]
struct Tokenizer {
    tokenizer: bytepiece_rs::Tokenizer,
}


#[pymethods]
impl Tokenizer {
    #[new]
    fn new(model_path: &str) -> Self {
        match model_path {
            "" => Self {
                tokenizer: bytepiece_rs::Tokenizer::new(),
            },
            _ => Self {
                tokenizer: bytepiece_rs::Tokenizer::load_from(model_path),
            },
        }
    }

    #[pyo3(text_signature = "($self, text, add_bos, add_eos, alpha)")]
    fn encode<'a>(
        &self, py: Python, text: &'a str, add_bos: bool, add_eos: bool, alpha: f64
    ) -> Vec<usize> {
        py.allow_threads(move || self.tokenizer.encode(text, add_bos, add_eos, alpha))
    }

    #[pyo3(text_signature = "($self, token_ids)")]
    fn decode<'a>(&self, py: Python, token_ids: Vec<usize>) -> String {
        py.allow_threads(move || self.tokenizer.decode(token_ids))
    }
}


#[pymodule]
fn rs_bytepiece(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    Ok(())
}