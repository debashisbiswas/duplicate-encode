use rand::{distributions::Alphanumeric, prelude::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::time::Instant;

const NUMBER_OF_TEST_RUNS: usize = 10;
const INPUT_WORD_SIZE: usize = 1_000_000;
const INPUT_WORD_NUM: usize = 10;
const CHARS_PER_TEST: usize = NUMBER_OF_TEST_RUNS * INPUT_WORD_SIZE * INPUT_WORD_NUM;

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

fn main() {
    println!(
        "Counting {} characters per test (over {} tests)",
        CHARS_PER_TEST, NUMBER_OF_TEST_RUNS
    );

    print!("Generating random input...");
    let start = Instant::now();
    let random = StdRng::seed_from_u64(42);

    let input_word_chunk: String = random
        .sample_iter(&Alphanumeric)
        .take(INPUT_WORD_SIZE)
        .map(char::from)
        .collect();
    let input_word = input_word_chunk.repeat(INPUT_WORD_NUM);
    let input_word = &input_word.as_str();
    println!(" took {} ms.", start.elapsed().as_millis());

    print!("Running tests...");
    let start = Instant::now();
    for _ in 0..NUMBER_OF_TEST_RUNS {
        duplicate_encode(input_word);
    }
    let finish = start.elapsed();
    println!(" took {} seconds.", finish.as_secs_f32());
}
