from collections import Counter, defaultdict


def oneline_list(word):
    """List comp version."""
    return "".join(["(" if word.count(c) == 1 else ")" for c in word])


def oneline_gen(word):
    """List comp version."""
    return "".join("(" if word.count(c) == 1 else ")" for c in word)


def oneline_vars(text):
    """Combined version."""
    counts = Counter(text)
    return "".join([')' if counts[t] - 1 else '(' for t in text])


def bash(word):
    """Bash's version."""
    counter = {}

    for char in word:
        counter[char] = counter.get(char, 0) + 1

    new_word = str()
    for char in word:
        new_word += '(' if counter[char] == 1 else ')'

    return new_word


def bash_single_update(word):
    """Bash's version - single update algorithm."""
    counter = {}

    for char in word:
        if char not in counter:
            counter[char] = 1
        elif counter[char] == 1:
            counter[char] = 2

    new_word = str()
    for char in word:
        new_word += '(' if counter[char] == 1 else ')'

    return new_word


def bash_single_update_str_join_instead_of_concat(word):
    """Bash's version - single update algorithm. using string.join instead
    of __radd__.
    """
    counter = {}

    for char in word:
        if char not in counter:
            counter[char] = 1
        elif counter[char] == 1:
            counter[char] = 2

    return "".join(['(' if counter[char] == 1 else ')' for char in word])


def bash_single_lookup_and_update_str_join_instead_of_concat(word):
    """Bash's version - single lookup and update algorithm. using
    string.join instead of __radd__.
    """
    counter = {}

    for char in word:
        match counter.get(char):
            case None:
                counter[char] = 1
            case 1:
                counter[char] = 2

    return "".join(['(' if counter[char] == 1 else ')' for char in word])


def bash_improved(text):
    """Bash's "improved" version.

    I tried using a dict comp here, but you can't reference yourself inside
    of a comprehension, nor assign to an expression so the `a,b=a[b]={},5`
    trick won't work. "Normal" dict it is...
    """
    counter = defaultdict(int)
    for char in text:
        counter[char] += 1

    return "".join([')' if counter[char] - 1 else '(' for char in text])


def bash_improved_single_update(text):
    """Bash's "improved" (again) version."""
    counter = defaultdict(int)
    for char in text:
        if char not in counter:
            counter[char]
            continue
        if counter.get(char):
            continue
        counter[char] = 1

    return "".join([')' if counter[char] else '(' for char in text])
