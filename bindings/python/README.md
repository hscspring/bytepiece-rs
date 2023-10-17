# rs-bytepiece

## Install

```bash
pip install rs_bytepiece
```

## Usage

```python
from rs_bytepiece import Tokenizer

tokenizer = Tokenizer()
# a custom model
tokenizer = Tokenizer("/path/to/model")
ids = tokenizer.encode("今天天气不错")
text = tokenizer.decode(ids)
```

## Performance

The performance is a bit faster than the original implementation. I've tested (on my M2 16G) the《鲁迅全集》which has 625890 chars. The time unit is millisecond.

| length | jieba    | aho_py  | aho_cy | aho_rs |
| ------ | -------- | ------- | ------ | ------ |
| 100    | 17062.12 | 1404.37 | 564.31 | 112.94 |
| 1000   | 17104.38 | 1424.6  | 573.32 | 113.18 |
| 10000  | 17432.58 | 1429.0  | 574.93 | 110.03 |
| 100000 | 17228.17 | 1401.01 | 574.5  | 110.44 |
| 625890 | 17305.95 | 1419.79 | 567.78 | 108.54  |

