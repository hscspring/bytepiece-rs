from rs_bytepiece import Tokenizer


tk = Tokenizer()


def test_tokenizer():
    text = "今天天气不错"
    ids = tk.encode(text)
    assert len(ids) == 3