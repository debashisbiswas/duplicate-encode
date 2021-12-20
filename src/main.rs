use counter::Counter;
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

fn duplicate_encode(text: &str) -> String {
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

fn duplicate_encode_capacity(text: &str) -> String {
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        if counter.contains_key(&c) {
            *counter.get_mut(&c).unwrap() += 1;
        } else {
            counter.insert(c, 1);
        }
    }

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = *counter.get(&c).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
}

fn duplicate_encode_counter(text: &str) -> String {
    let char_counts = text.chars().collect::<Counter<_>>();
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = char_counts[&c];
        result.push(if count == 1 { '(' } else { ')' });
    }
    return result;
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

    println!("Running tests...");
    for f in functions {
        let start = Instant::now();
        for _ in 0..NUMBER_OF_TEST_RUNS {
            (f.body)(input_word);
        }
        let finish = start.elapsed();
        println!("{} took {} seconds.", f.name, finish.as_secs_f32());
    }
}

fn main() {
    let functions = Vec::from([
        NamedFunction {
            name: "duplicate_encode",
            body: duplicate_encode,
        },
        NamedFunction {
            name: "duplicate_encode_capacity",
            body: duplicate_encode_capacity,
        },
        NamedFunction {
            name: "duplicate_encode_counter",
            body: duplicate_encode_counter,
        },
    ]);
    test_functions(functions);
}
