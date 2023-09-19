# -*- coding: utf-8 -*-
# Reference: https://kexue.fm/archives/9752

import numpy as np
import re, json, unicodedata
from itertools import chain
from functools import partial
from tqdm import tqdm, trange
from base64 import b64encode, b64decode
from multiprocessing import Pool, Queue
import ahocorasick


def normalize(text, maxlen=0):
    if not isinstance(text, bytes):
        text = unicodedata.normalize('NFC', text).encode()
    if maxlen > 0:
        return re.findall(b'.{,%d}\n{1,100}|.{1,%d}' % (maxlen, maxlen), text)
    else:
        return re.findall(b'.*\n+|.+', text)



class Tokenizer:
    """Unigram tokenizer with Aho-Corasick automaton
    """
    def __init__(self, pieces):
        if isinstance(pieces, str):
            pieces = json.load(open(pieces))
        pieces = {b64decode(k): v for k, v in pieces.items()}
        self._pieces = {k: v[-1] for k, v in pieces.items()}
        self._piece2id = {k: v[0] for k, v in pieces.items()}
        for i, k in enumerate(['<pad>', '<bos>', '<eos>']):
            self._piece2id[k] = i
        self._id2piece = {v: k for k, v in self._piece2id.items()}
        self.vocab_size = len(self._pieces) + 3
        # Aho-Corasick automaton
        log_total = np.log(sum(self._pieces.values()))
        self._automaton = ahocorasick.Automaton()
        for k, v in self._pieces.items():
            self._automaton.add_word(k, (len(k), np.log(v) - log_total))
        self._automaton.make_automaton()

    def _tokenize(self, text):
        routes = [(0, 0)] + [(-np.inf, 0) for _ in text]
        tokens = []
        for end, (_len, _score) in self._automaton.iter(text):
            start = end - _len + 1
            end = end + 1
            score = routes[start][0] + _score
            if score > routes[end][0]:
                routes[end] = score, start
        end = len(text)
        while text:
            start = routes[end][1]
            tokens.append(text[start:end])
            text = text[:start]
            end = start
        return tokens[::-1]

    def tokenize(self, text):
        return list(chain(*[self._tokenize(t) for t in normalize(text)]))

    def piece_to_id(self, p):
        return self._piece2id[p]

    def id_to_piece(self, i):
        return self._id2piece[i]

    def pieces_to_ids(self, pieces):
        return [self._piece2id[p] for p in pieces]

    def ids_to_pieces(self, ids):
        return [self._id2piece[i] for i in ids]

    def encode(self, text, add_bos=False, add_eos=False):
        pieces = [self._piece2id[p] for p in self.tokenize(text)]
        if add_bos:
            pieces = [1] + pieces
        if add_eos:
            pieces += [2]
        return pieces

    def decode(self, ids):
        pieces = [self._id2piece[i] for i in ids if i > 2]
        return b''.join(pieces).decode(errors='ignore')
