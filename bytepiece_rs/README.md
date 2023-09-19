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

| StringLength | alpha | AverageTime | Throughput   |
| ------------ | ----- | ----------- | ------------ |
| 100          | 0.0   | 55.411 µs   | 1.7211 MiB/s |
|              | 0.1   | 68.615 µs   | 1.3899 MiB/s |
| 1000         | 0.0   | 804.22 µs   | 1.1858 MiB/s |
|              | 0.1   | 989.76 µs   | 986.66 KiB/s |
| 10000        | 0.0   | 8.3846 ms   | 1.1374 MiB/s |
|              | 0.1   | 10.811 ms   | 903.34 KiB/s |
| 100000       | 0.0   | 85.445 ms   | 1.1161 MiB/s |
|              | 0.1   | 110.75 ms   | 881.77 KiB/s |

