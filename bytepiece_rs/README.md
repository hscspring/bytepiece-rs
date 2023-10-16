# bytepiece-rs

## Usage

```rust
use bytepice_rs::Tokenizer;

let tokenizer = Tokenizer::new();
// or load a custom model
let tokenizer = Tokenizer::load_from("/path/to/model");
let text = "今天天气不错";
let ids = tokenizer.encode(text, false, false, alpha=0.0);
assert_eq!(ids, vec![40496, 45268, 39432]);
let text2 = tokenizer.decode(ids);
assert_eq!(text2, text);
```


## Benchmark  & Test

```bash
cargo test
cargo bench -- --plotting-backend gnuplot
```
