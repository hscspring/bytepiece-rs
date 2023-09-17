import json
from os.path import dirname, join
import unicodedata
import time
from base64 import b64decode

import ahocorasick


def main():
    root = dirname(dirname(__file__))
    model_path = join(root, "src/tokenizer/model/bytepiece_80k.model")
    with open(model_path, "r") as f:
        dct = json.load(f)
    patterns = []
    for key in dct:
        pattern = b64decode(key)
        patterns.append(pattern)
    aho = ahocorasick.Automaton()
    for p in patterns:
        aho.add_word(p, p)
    aho.make_automaton()

    with open("data/鲁迅全集.txt", "r") as f:
        text = f.read()
    
    normalized_text = unicodedata.normalize("NFC", text)
    for total in [100, 1000, 10000, 100000]:
        text = normalized_text[:total]
        byte_text = text.encode()
        start_time = time.time()
        count = 0
        for end_index, original_value in aho.iter(byte_text):
            count += 1
        end_time = time.time()
        elapsed_time = (end_time - start_time) * 1000;
        print(f"Count: {total} ms, Elapsed time: {elapsed_time}")

    

if __name__ == "__main__":
    main()