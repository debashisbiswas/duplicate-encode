import random
from datetime import datetime
from functools import partial

import pytest

import duplicate_encode

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
    duplicate_encode.oneline_list,
    duplicate_encode.oneline_gen,
    duplicate_encode.oneline_vars,
    duplicate_encode.bash,
    duplicate_encode.bash_single_update,
    duplicate_encode.bash_single_update_str_join_instead_of_concat,
    duplicate_encode.bash_single_lookup_and_update_str_join_instead_of_concat,
    duplicate_encode.bash_improved,
    duplicate_encode.bash_improved_single_update,
])
def test_funcs(func, input_text, benchmark):
    result = benchmark(func, input_text)

    # Every character is always repeated because the input_text is chunked.
    assert result == ")" * (INPUT_TEXT_CHUNK_SIZE * NUM_INPUT_TEXT_CHUNKS)


if __name__ == "__main__":
    pytest.main()
