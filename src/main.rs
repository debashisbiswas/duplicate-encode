use counter::Counter;
use itertools::Itertools;
use rand::distributions::Uniform;
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::time::Instant;

const NUMBER_OF_TEST_RUNS: usize = 10;
const INPUT_WORD_SIZE: usize = 1_000_000;
const INPUT_WORD_NUM: usize = 10;
const CHARS_PER_TEST: usize = NUMBER_OF_TEST_RUNS * INPUT_WORD_SIZE * INPUT_WORD_NUM;

struct NamedFunction {
    name: &'static str,
    body: fn(&str) -> String,
}

// The initial implementation.
fn duplicate_encode(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        if counter.contains_key(&c) {
            *counter.get_mut(&c).unwrap() += 1;
        } else {
            counter.insert(c, 1);
        }
    }

    let mut result = String::new();
    for c in text.chars() {
        let count = *counter.get(&c).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
}

// Similar to the duplicate_encode function, but using a different method of
// populating the HashMap when counting elements at the beginning.
// This method is noticably faster.
fn duplicate_encode_better_insertion(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    let mut result = String::new();
    for c in text.chars() {
        let count = *counter.get(&c).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
}

// Similar to duplicate_encode_better_insertion, but using String::with_capacity
// instead of String::new to allocate space for the whole result once.
// This seems to be slightly faster.
fn duplicate_encode_capacity(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = *counter.get(&c).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
}

// Similar to duplicate_encode_capacity, but using Counter from the counter
// crate to count elements, rather than using a HashMap.
// This seems to be slower than the implementations that utilize a HashMap,
// including the itertools version (which uses a HashMap internally)
fn duplicate_encode_counter(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.chars().collect::<Counter<_>>();

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = counts[&c];
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
}

// Similar to duplicate_encode_counter, but using itertools to count the elements.
// Note that itertools also uses a HashMap.
// This is faster than using a Counter, and provides similar performance to
// the other versions that also use a HashMap.
fn duplicate_encode_itertools(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.chars().counts();
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = counts[&c];
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
}

// Similar to duplicate_encode_counter, but using the map function to build
// the final string using an iterator.
// If anything, this is only slightly slower. This might be because the space
// for the String built at the end is not allocated in advance.
fn duplicate_encode_map(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.chars().counts();
    text.chars()
        .map(|c| if counts[&c] == 1 { '(' } else { ')' })
        .collect()
}

fn test_functions(functions: Vec<NamedFunction>) {
    println!(
        "Counting {} characters per test (over {} tests)",
        CHARS_PER_TEST, NUMBER_OF_TEST_RUNS
    );

    print!("Generating random input...");
    let start = Instant::now();
    let random = StdRng::seed_from_u64(42);
    let range = Uniform::new_inclusive(b'0', b'z');
    let input_word_chunk = random
        .sample_iter(&range)
        .take(INPUT_WORD_SIZE)
        .map(char::from)
        .collect::<String>()
        .repeat(INPUT_WORD_NUM);
    let input_word = &input_word_chunk.as_str();
    println!(" took {} ms.", start.elapsed().as_millis());

    // Get the longest function name in the list for formatted printing.
    let mut longest_name_len = 0;
    for f in functions.iter() {
        let len = f.name.len();
        if len > longest_name_len {
            longest_name_len = len;
        }
    }

    for f in functions.iter() {
        let start = Instant::now();
        for _ in 0..NUMBER_OF_TEST_RUNS {
            (f.body)(input_word);
        }
        let finish = start.elapsed();
        println!(
            "{:<max_len$} | {1:.3} seconds",
            f.name,
            finish.as_secs_f32(),
            max_len = longest_name_len
        );
    }
}

fn main() {
    let functions = Vec::from([
        NamedFunction {
            name: "duplicate_encode",
            body: duplicate_encode,
        },
        NamedFunction {
            name: "duplicate_encode_better_insertion",
            body: duplicate_encode_better_insertion,
        },
        NamedFunction {
            name: "duplicate_encode_capacity",
            body: duplicate_encode_capacity,
        },
        NamedFunction {
            name: "duplicate_encode_counter",
            body: duplicate_encode_counter,
        },
        NamedFunction {
            name: "duplicate_encode_itertools",
            body: duplicate_encode_itertools,
        },
        NamedFunction {
            name: "duplicate_encode_map",
            body: duplicate_encode_map,
        },
    ]);
    test_functions(functions);
}
