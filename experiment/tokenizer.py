import numpy as np
import re, json, unicodedata
from itertools import chain
from functools import partial
from tqdm import tqdm, trange
from base64 import b64encode, b64decode
from multiprocessing import Pool, Queue
import ahocorasick

pieces = "../bytepiece_rs/src/tokenizer/model/bytepiece_80k.model"
pieces = json.load(open(pieces))
pieces = {b64decode(k): v for k, v in pieces.items()}
_pieces = {k: v[-1] for k, v in pieces.items()}
_piece2id = {k: v[0] for k, v in pieces.items()}
for i, k in enumerate(['<pad>', '<bos>', '<eos>']):
    _piece2id[k] = i
_id2piece = {v: k for k, v in _piece2id.items()}
vocab_size = len(_pieces) + 3
log_total = np.log(sum(_pieces.values()))
_automaton = ahocorasick.Automaton()
for k, v in _pieces.items():
    _automaton.add_word(k, (len(k), np.log(v) - log_total))
_automaton.make_automaton()
max_piece_len = max([len(v) for v in _pieces])


res_tokens = []

def step(routes, start, text, last_locs, last=False):
    
    for end, v in enumerate(routes):
        if v[0] == -np.inf:
            end -= 1
            break
    
    locs = []
    s, e = start, end
    actual_end = e
    while e > start:
        s = routes[e][1]
        locs.append((s, e))
        e = s
    
    if last_locs and locs:
        ns = locs[-1][0]
        for s,e in last_locs[::-1]:
            if s >= ns:
                ns, ne = locs.pop()
                token = text[s: ne].decode("utf8")
                print(token)
                res_tokens.append(token)
                actual_end = ne
                break
            token = text[s:e].decode("utf8")
            print(token)
            res_tokens.append(token)
    
    if last:
        for s,e in locs[::-1]:
            token = text[s:e].decode("utf8")
            print(token)
            res_tokens.append(token)
    
    return locs, actual_end + 1, end - actual_end


def tokenize(text):
    routes = [(0, 0)] + [(-np.inf, 0) for _ in range(len(text))]
    count = 1
    last_end = np.inf
    nxt_start = 0
    last_locs = []
    for end, (_len, _score) in _automaton.iter(text):
        start = end - _len + 1
        end = end + 1
        score = routes[start][0] + _score
        if score > routes[end][0]:
            routes[end] = (score, start)

        if last_end < end:
            if count == max_piece_len:
                last_locs, nxt_start, remain = step(routes, nxt_start, text, last_locs)
                count = remain
            count += 1
        last_end = end
    last_locs, nxt_start, remain = step(routes, nxt_start, text, last_locs, True)


text = "我们知道，为了使得卷积编码过程中的 feature 保持一定的大小，我们通常会对输入 padding 一定的 0，而这篇论文显示该操作导致模型有能力识别位置信息。也就是说，卷积核的各向异性固然重要，但是最根本的是 zero padding 的存在，那么可以想象，实际上提取的是当前位置与 padding 的边界的相对距离。"
text_utf8 = text.encode("utf8")
tokenize(text_utf8)

assert "".join(res_tokens) == text