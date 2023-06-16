# %%
from typing import List

from attr import dataclass

@dataclass
class Metapath:
    name:str
    metapath : List[List[str]]
