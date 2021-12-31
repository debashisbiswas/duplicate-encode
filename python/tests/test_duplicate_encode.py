import random
from datetime import datetime
from functools import partial

import pytest

from duplicate_encode import DuplicateEncode

INPUT_TEXT_CHUNK_SIZE = 10_000
NUM_INPUT_TEXT_CHUNKS = 10


@pytest.fixture(scope='module')
def input_text():
    random.seed(42)
    input_text_chunk = "".join(
        [random.choice([chr(x) for x in range(ord("0"), ord("z"))]) for _ in range(INPUT_TEXT_CHUNK_SIZE)])
    input_text = input_text_chunk * NUM_INPUT_TEXT_CHUNKS

    yield input_text


@pytest.mark.parametrize("func", [
    DuplicateEncode.oneline_list,
    DuplicateEncode.oneline_gen,
    DuplicateEncode.bash,
    DuplicateEncode.bash_single_update_str_join_instead_of_concat,
    DuplicateEncode.bash_single_update,
    DuplicateEncode.bash_improved,
    DuplicateEncode.bash_improved_single_update,
    DuplicateEncode.oneline_vars,
])
def test_funcs(func, input_text, benchmark):
    result = benchmark(func, input_text)

    # Every character is always repeated because the input_text is chunked.
    assert result == ")" * (INPUT_TEXT_CHUNK_SIZE * NUM_INPUT_TEXT_CHUNKS)


if __name__ == "__main__":
    pytest.main()
