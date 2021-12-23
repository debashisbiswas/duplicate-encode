def duplicate_encode_oneline_list(word):
    """List comp version."""
    return "".join(["(" if word.count(c) == 1 else ")" for c in word])  ##FIXME remove '[]' (and/or test)


def duplicate_encode_oneline_gen(word):
    """List comp version."""
    return "".join("(" if word.count(c) == 1 else ")" for c in word)


from collections import Counter
def duplicate_encode_oneline_vars(text):
    """Combined version."""
    counts = Counter(text)
    return "".join([')' if counts[t]-1 else '(' for t in text])


def duplicate_encode_bash(word):
    """Bash's version."""
    counter = {}

    for char in word:
        counter[char] = counter.get(char, 0) + 1

    new_word = str()
    for char in word:  #FIXME   str comp here
        new_word += '(' if counter[char] == 1 else ')'

    return new_word


def duplicate_encode_bash_single_update(word):
    """Bash's version - single update algorithm."""
    counter = {}

    for char in word:
        if char not in counter:
            counter[char] = 1
        elif counter[char] == 1:
            counter[char] = 2

    new_word = str()
    for char in word:  #FIXME   str comp here
        new_word += '(' if counter[char] == 1 else ')'

    return new_word

def duplicate_encode_bash_single_update_str_join_instead_of_concat(word):
    """Bash's version - single update algorithm. using string.join instead of __radd__"""
    counter = {}

    for char in word:
        if char not in counter:
            counter[char] = 1
        elif counter[char] == 1:
            counter[char] = 2

    return "".join(['(' if counter[char] == 1 else ')' for char in word])


from collections import defaultdict
def duplicate_encode_bash_improved(text):
    """Bash's "improved" version.

    I tried using a dict comp here, but you can't reference yourself inside
    of a comprehension, nor assign to an expression so the `a,b=a[b]={},5`
    trick won't work. "Normal" dict it is...
    """
    counter = defaultdict(int)
    for char in text:
        counter[char] += 1

    return "".join([')' if counter[char]-1 else '(' for char in text])

from collections import defaultdict
def duplicate_encode_bash_improved_single_update(text):
    """Bash's "improved" (again) version.

    I tried using a dict comp here, but you can't reference yourself inside
    of a comprehension, nor assign to an expression so the `a,b=a[b]={},5`
    trick won't work. "Normal" dict it is...
    """
    counter = defaultdict(int)
    for char in text:
        if char not in counter:
            counter[char]
            continue
        if counter.get(char):
            continue
        counter[char] = 1

    return "".join([')' if counter[char] else '(' for char in text])



number_of_test_runs=10
input_word_size = 1_000_000
# input_word_size = 1_000
input_word_num = 10

chars_per_test = number_of_test_runs * input_word_size * input_word_num
skip_slow_algos = chars_per_test > 1_000_000

print(f"counting {chars_per_test:,} characters per test (over {number_of_test_runs} tests for 5 functions")


import random
from datetime import datetime
# seed randomizer with static value. if we run this more than once, then the
# input will change.
random.seed(42)
start = datetime.now()
print("making a random number...")
input_word_chunk = "".join([random.choice([chr(x) for x in range(ord("0"), ord("z"))]) for _ in range(input_word_size)])
input_word = input_word_chunk * input_word_num
print(f"done makinga random number (took {datetime.now() - start})")


from timeit import timeit
from functools import partial
oneliner_list  = partial(duplicate_encode_oneline_list,  input_word)
oneliner_gen   = partial(duplicate_encode_oneline_gen,   input_word)
debasheses     = partial(duplicate_encode_bash,          input_word)
debasheses_su_join = partial(duplicate_encode_bash_single_update_str_join_instead_of_concat,          input_word)
debasheses_su  = partial(duplicate_encode_bash_single_update, input_word)
debasheses_imp = partial(duplicate_encode_bash_improved, input_word)
debasheses_imp_su = partial(duplicate_encode_bash_improved_single_update, input_word)
oneliner_vars  = partial(duplicate_encode_oneline_vars,  input_word)

oneline_list, oneline_gen = None, None
if not skip_slow_algos:
    oneline_list  = timeit(oneliner_list,  number=number_of_test_runs)
    oneline_gen   = timeit(oneliner_gen,   number=number_of_test_runs)
debashis      = timeit(debasheses,     number=number_of_test_runs)
debashis_su_join = timeit(debasheses_su_join,     number=number_of_test_runs)
debashis_su   = timeit(debasheses_su,  number=number_of_test_runs)
debashis_imp  = timeit(debasheses_imp, number=number_of_test_runs)
debashis_imp_su  = timeit(debasheses_imp_su, number=number_of_test_runs)
oneline_vars  = timeit(oneliner_vars,  number=number_of_test_runs)

print("output sorted by speed on tener's computer")
if not skip_slow_algos:
    print(f"One-liner list:    {oneline_list} seconds")
    print(f"One-liner gen:     {oneline_gen} seconds")
else:
    factor = chars_per_test / 10_000
    # print(f"One-liner list:    skipped.............estimate: {6*factor/3600} hours")  these estimates are for an old version on a specific computer.. not good.
    # print(f"One-liner gen:     skipped.............estimate: {9*factor/3600} hours")
    print(f"One-liner list:    skipped; test is too big")
    print(f"One-liner gen:     skipped; test is too big")
print(f"Debashis:                        {debashis} seconds")
print(f"Debashis single update:          {debashis_su} seconds")
print(f"Debashis single update join:     {debashis_su_join} seconds")
print(f"Debashis improved:               {debashis_imp} seconds")
print(f"Debashis improved single update: {debashis_imp_su} seconds")
print(f"One-liner vars:                  {oneline_vars} seconds")

# correctness
control_output, generator_output = None, None
if not skip_slow_algos:
    control_output       = duplicate_encode_oneline_list(input_word)
    generator_output     = duplicate_encode_oneline_gen(input_word)
debashis_output      = duplicate_encode_bash(input_word)
debashis_output_su_join = duplicate_encode_bash_single_update_str_join_instead_of_concat(input_word)
debashis_su_output   = duplicate_encode_bash_single_update(input_word)
debashis_imp_output  = duplicate_encode_bash_improved(input_word)
debashis_imp_su_output  = duplicate_encode_bash_improved_single_update(input_word)
vars_output          = duplicate_encode_oneline_vars(input_word)

if skip_slow_algos:
    assert debashis_output == debashis_imp_output == vars_output == debashis_su_output == debashis_imp_su_output == debashis_output_su_join
else:
    assert control_output == generator_output == debashis_output == debashis_imp_output == vars_output == debashis_su_output == debashis_imp_su_output == debashis_output_su_join
