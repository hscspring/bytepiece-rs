# bytepiece-rs

## Usage

```rust
use bytepice_rs::Tokenizer;

let tokenizer = Tokenizer::new();
let text = "今天天气不错";
let ids = tokenizer.encode(text, false, false);
assert_eq!(ids, vec![40496, 45268, 39432]);
let text2 = tokenizer.decode(ids);
assert_eq!(text2, text);
```