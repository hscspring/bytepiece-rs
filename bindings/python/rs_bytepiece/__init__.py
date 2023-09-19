from typing import Optional, List

from .rs_bytepiece import Tokenizer as BytepieceTokenizer


class Tokenizer:

    def __init__(self, model_path: str = ""):
        self.tokenizer = BytepieceTokenizer(model_path)
    
    def load_from(self, model_path: str):
        self.tokenizer.load_from(model_path)
    
    def encode(
        self, 
        text: str, 
        add_bos: bool = False, 
        add_eos: bool = False, 
        alpha: float = 0.0,
    ) -> List[int]:
        return self.tokenizer.encode(text, add_bos, add_eos, alpha)
    
    def decode(
        self,
        ids: List[int],
    ) -> str:
        return self.tokenizer.decode(ids)


__version__ = "0.0.3"