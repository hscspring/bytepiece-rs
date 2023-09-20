# rs-bytepiece

## Install

```bash
pip install rs_bytepiece
```

## Usage

```python
from rs_bytepiece import Tokenizer

tokenizer = Tokenizer()
ids = tokenizer.encode("今天天气不错")
text = tokenizer.decode(ids)
```

## Performance

The performance is a bit faster than the original implementation. I've tested the《鲁迅全集》which has 625890 chars. The time unit is millisecond.

| length | jieba    | aho_py  | aho_cy | aho_rs |
| ------ | -------- | ------- | ------ | ------ |
| 100    | 17062.12 | 1404.37 | 564.31 | 299.09 |
| 1000   | 17104.38 | 1424.6  | 573.32 | 281.84 |
| 10000  | 17432.58 | 1429.0  | 574.93 | 293.16 |
| 100000 | 17228.17 | 1401.01 | 574.5  | 280.81 |
| 625890 | 17305.95 | 1419.79 | 567.78 | 282.35 |

