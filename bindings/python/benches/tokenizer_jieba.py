from typing import List, Dict, Tuple, Any

import json
import unicodedata
from base64 import b64decode
import re
from math import log


reg_bytes = re.compile(rb'.*\n+|.+')


class Tokenizer:

    def __init__(self, model_file: str):
        self.special_pieces = [
            b'<pad>', b'<bos>', b'<eos>',
        ]
        with open(model_file, "r") as f:
            dct = json.load(f)
        self.p2id = {}
        self.p2freq = {}
        for i, k in enumerate(self.special_pieces):
            self.p2id[k] = i
        for bs64_token, val_list in dct.items():
            piece = b64decode(bs64_token)
            self.p2id[piece] = val_list[0]
            self.p2freq[piece] = val_list[2]
        self.id2p = dict(zip(self.p2id.values(), self.p2id.keys()))
        self.vocab_size = len(self.p2id)
        log_total = log(sum(self.p2freq.values()))
        self.p2score = {k: log(v) - log_total for k, v in self.p2freq.items()}

        self.max_piece_len, self.max_non_eng_len = self.get_max_non_eng_len(
            self.p2id)

    def get_max_non_eng_len(self, token_list: List[bytes]):
        max_non_eng_len = 0
        max_piece_len = 0
        for v in token_list:
            _len = len(v)
            if _len > max_piece_len:
                max_piece_len = _len
            flag = True
            for i in v:
                # english
                if i <= 127:
                    flag = False
                    break
            # all non-english
            if flag:
                if _len > max_non_eng_len:
                    max_non_eng_len = _len
        return max_piece_len, max_non_eng_len

    def normalize(self, text: str):
        utf8 = unicodedata.normalize('NFC', text).encode()
        return reg_bytes.findall(utf8)

    def get_dag(self, bsent: bytes) -> Dict[int, List[int]]:
        dag = {}
        lens = len(bsent)
        for k in range(lens):
            tmplist = []
            i = k
            frag = bsent[k:k + 1]
            while i < lens:
                if frag in self.p2id:
                    tmplist.append(i)
                i += 1
                frag = bsent[k: i + 1]
                if i + 1 - k > self.max_piece_len:
                    break
                if (
                    i + 1 - k > self.max_non_eng_len and
                    all(b > 127 for b in frag)
                ):
                    break
            if not tmplist:
                tmplist.append(k)
            dag[k] = tmplist
        return dag

    def get_route(
        self,
        bsent: bytes,
        dag: Dict[int, List[int]]
    ) -> Dict[int, Tuple[float, int]]:
        route = {}
        lens = len(bsent)
        route[lens] = (0, 0)
        for idx in range(lens - 1, -1, -1):
            tmp = []
            for i in dag[idx]:
                key = bsent[idx: i + 1]
                val = self.p2score.get(key, 0.0) + route[i + 1][0]
                tmp.append((val, i))
            route[idx] = self.get_max(tmp)
        return route

    def get_max(self, lst: List[Tuple]) -> List[Tuple]:
        maxv = lst[0]
        for (score, idx) in lst:
            if score > maxv[0]:
                maxv = (score, idx)
        return maxv

    def _tokenize(self, bsent: bytes) -> List[bytes]:
        dag = self.get_dag(bsent)
        route = self.get_route(bsent, dag)
        start = 0
        lens = len(bsent)
        tokens = []
        while start < lens:
            end = route[start][1] + 1
            piece = bsent[start: end]
            tokens.append(piece)
            start = end
        return tokens

    def tokenize(self, text: str) -> List[bytes]:
        tokens = []
        bytes_sent_list = self.normalize(text)
        for bs in bytes_sent_list:
            part_pieces = self._tokenize(bs)
            for piece in part_pieces:
                tokens.append(piece)
        return tokens

    def piece_to_id(self, p: bytes) -> int:
        return self.p2id[p]

    def id_to_piece(self, i: int) -> bytes:
        return self.id2p[i]

    def pieces_to_ids(self, pieces: List[bytes]) -> List[int]:
        return [self.p2id[p] for p in pieces]

    def ids_to_pieces(self, ids: List[int]) -> List[bytes]:
        return [self.id2p[i] for i in ids]

    def encode(self, text: str, add_bos=False, add_eos=False) -> List[int]:
        byte_tokens = self.tokenize(text)
        ids = [self.p2id[p] for p in byte_tokens]
        if add_bos:
            ids = [1] + ids
        if add_eos:
            ids += [2]
        return ids

    def decode(self, ids: List[int]) -> str:
        pieces = [self.id2p[i] for i in ids if i > 2]
        return b''.join(pieces).decode(errors='ignore')
