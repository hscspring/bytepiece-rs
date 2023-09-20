import time
from functools import wraps
from os.path import abspath, dirname
from pathlib import Path


from tokenizer_jieba import Tokenizer as TokenizerJieba
from tokenizer_aho import Tokenizer as TokenizerAhoPython
from bytepiece import Tokenizer as TokenizerAhoCython
from rs_bytepiece import Tokenizer as TokenizerRs


def timethis(func):

    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        cost = round((end - start) * 1000, 2)
        print(func.__name__, cost)
        return result
    return wrapper


root = Path(dirname(dirname(dirname(dirname(abspath(__file__))))))


model_file = root / "bytepiece_rs/src/tokenizer/model/bytepiece_80k.model"
text_file = root / "bytepiece_rs/bench_aho/data/鲁迅全集.txt"

model_file = str(model_file)

tk_jieba = TokenizerJieba(model_file)
tk_aho_py = TokenizerAhoPython(model_file)
tk_aho_cy = TokenizerAhoCython(model_file)
tk_rs = TokenizerRs(model_file)


with open(text_file, "r") as f:
    text = f.read()


@timethis
def run_tk_jieba():
    ids = tk_jieba.encode(text)
    return ids

@timethis
def run_tk_aho_py():
    ids = tk_aho_py.encode(text)
    return ids


@timethis
def run_tk_aho_cy():
    ids = tk_aho_cy.encode(text)
    return ids


@timethis
def run_tk_rs():
    ids = tk_rs.encode(text)
    return ids


for total in [100, 1000, 10000, 100000, 1000000]:
    test_text = text[:total]
    _len = len(test_text)
    print(f"\nText length: {_len}")
    # ids0 = run_tk_jieba()
    ids1 = run_tk_aho_py()
    ids2 = run_tk_aho_cy()
    ids3 = run_tk_rs()

    assert len(ids1) == len(ids2) == len(ids3)
    for i in range(len(ids1)):
        if ids1[i] == ids2[i] == ids3[i]:
            continue
        print(i)
