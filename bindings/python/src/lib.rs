use pyo3::prelude::*;


#[pyclass(subclass)]
struct Tokenizer {
    tokenizer: bytepiece_rs::Tokenizer,
}


#[pymethods]
impl Tokenizer {
    #[new]
    #[pyo3[signature = (model_path="")]]
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

    #[pyo3(signature = (text, add_bos=false, add_eos=false, alpha=-1.0, norm=true))]
    fn encode<'a>(
        &self, py: Python, text: &'a str, add_bos: bool, add_eos: bool, alpha: f64, norm: bool
    ) -> Vec<usize> {
        py.allow_threads(move || self.tokenizer.encode(text, add_bos, add_eos, alpha, norm))
    }

    #[pyo3(signature = (token_ids))]
    fn decode<'a>(&self, py: Python, token_ids: Vec<usize>) -> String {
        py.allow_threads(move || self.tokenizer.decode(token_ids))
    }
}


#[pymodule]
fn rs_bytepiece(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    Ok(())
}