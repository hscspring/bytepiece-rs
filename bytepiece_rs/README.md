# bytepiece-rs

## Usage

```rust
use bytepice_rs::Tokenizer;

let tokenizer = Tokenizer::new();
// or load a custom model
let tokenizer = Tokenizer::build_from(model_path);
let text = "今天天气不错";
let ids = tokenizer.encode(text, false, false, alpha=0.0);
assert_eq!(ids, vec![40496, 45268, 39432]);
let text2 = tokenizer.decode(ids);
assert_eq!(text2, text);
```


## Benchmark 

```bash
cargo bench -- --plotting-backend gnuplot
```

The result as follows:

| TextLength | alpha | v0.0.2 | v0.0.3 | v0.0.4 |
| ------------ | ----- | ----------- | ----------- | ----------- |
| 100          | 0.0   | 55.411 µs   | 35.141 µs | 28.156 µs |
|              | 0.1   | 68.615 µs   | 47.058 µs | 38.373 µs |
| 1000         | 0.0   | 804.22 µs   | 465.13 µs | 400.85 µs |
|              | 0.1   | 989.76 µs   | 644.85 µs | 561.81 µs |
| 10000        | 0.0   | 8.3846 ms   | 4.911 ms | 4.1963 ms |
|              | 0.1   | 10.811 ms   | 6.7284 ms | 5.8504 ms |
| 100000       | 0.0   | 85.445 ms   | 51.324 ms | 42.54 ms |
|              | 0.1   | 110.75 ms   | 68.991 ms | 59.018 ms |

