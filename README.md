# bytepiece

Implementation of Su's [bytepiece](https://github.com/bojone/bytepiece).


## Bindings

- [Rust](https://github.com/hscspring/bytepiece-rs/tree/main/bytepiece_rs)
- [Python](https://github.com/hscspring/bytepiece-rs/tree/main/bindings/python)


## Quick Example using Python

```python
from rs_bytepiece import Tokenizer

tokenizer = Tokenizer()
output = tokenizer.encode("今天天气不错")
print(output)
# []
```