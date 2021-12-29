use counter::Counter;
use itertools::Itertools;
use num_format::Locale;
use num_format::ToFormattedString;
use rand::distributions::Uniform;
use rand::{prelude::StdRng, Rng, SeedableRng};
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const NUMBER_OF_TEST_RUNS: usize = 10;
const INPUT_WORD_SIZE: usize = 1_000_000;
const INPUT_WORD_NUM: usize = 10;
const CHUNK_COUNT: usize = 10_000;
const OUTPUT_PRECISION: usize = 5;

const TOTAL_INPUT_SIZE: usize = INPUT_WORD_SIZE * INPUT_WORD_NUM;
const CHARS_PER_TEST: usize = TOTAL_INPUT_SIZE * INPUT_WORD_NUM;
const CHUNK_SIZE: usize = TOTAL_INPUT_SIZE / CHUNK_COUNT;

struct NamedFunction {
    name: &'static str,
    body: fn(&str) -> String,
    enabled: bool,
}

/// The initial implementation.
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
    result
}

/// Similar to the duplicate_encode function, but using a different method of
/// populating the HashMap when counting elements at the beginning.
/// This method is noticably faster.
fn duplicate_encode_default(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *counter.entry(c).or_default() += 1;
    }

    let mut result = String::new();
    for c in text.chars() {
        let count = *counter.get(&c).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    result
}

/// Similar to duplicate_encode_better_insertion, but using String::with_capacity
/// instead of String::new to allocate space for the whole result once.
/// This seems to be slightly faster.
fn duplicate_encode_capacity(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *counter.entry(c).or_default() += 1;
    }

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = *counter.get(&c).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    result
}

/// Similar to duplicate_encode_capacity, but rather than converting the
/// lower string into a lowercase version upfront, converts individual chars into
/// lowercase as needed.
/// No noticable difference in performance.
fn duplicate_encode_lower(text: &str) -> String {
    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *counter.entry(c.to_ascii_lowercase()).or_default() += 1;
    }

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = *counter.get(&c.to_ascii_lowercase()).unwrap();
        result.push(if count == 1 { '(' } else { ')' });
    }
    result
}

/// Similar to duplicate_encode_capacity, but using Counter from the counter
/// crate to count elements, rather than using a HashMap.
/// This seems to be slower than the implementations that utilize a HashMap,
/// including the itertools version (which uses a HashMap internally)
fn duplicate_encode_counter(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.chars().collect::<Counter<_>>();

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = counts[&c];
        result.push(if count == 1 { '(' } else { ')' });
    }
    result
}

/// Similar to duplicate_encode_counter, but using itertools to count the elements.
/// Note that itertools also uses a HashMap.
/// This is faster than using a Counter, and provides similar performance to
/// the other versions that also use a HashMap.
fn duplicate_encode_itertools(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.chars().counts();
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        let count = counts[&c];
        result.push(if count == 1 { '(' } else { ')' });
    }
    result
}

/// Similar to duplicate_encode_itertools, but using the map function to build
/// the final string using an iterator.
/// If anything, this is only slightly slower. This might be because the space
/// for the String built at the end is not allocated in advance.
fn duplicate_encode_map(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.chars().counts();
    text.chars()
        .map(|c| if counts[&c] == 1 { '(' } else { ')' })
        .collect()
}

/// Similar to duplicate_encode_map, but iterating over bytes instead of chars.
/// Note that as_bytes() is used rather than bytes() to avoid copying the whole
/// input unnecessarily.
/// Slightly slower than duplicate_encode_map.
fn duplicate_encode_bytes(text: &str) -> String {
    let text = text.to_ascii_lowercase();
    let counts = text.as_bytes().iter().counts();
    text.as_bytes()
        .iter()
        .map(|b| if counts[&b] == 1 { '(' } else { ')' })
        .collect()
}

/// Similar to duplicate_encode_bytes, but avoiding an extra allocation.
/// to_ascii_lowercase allocates space for a new String, and this method
/// updates this String in place before returning it.
/// Slightly slower than duplicate_encode_map, but faster than the
/// duplicate_encode_bytes function.
fn duplicate_encode_in_place(text: &str) -> String {
    let mut text = text.to_ascii_lowercase();
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for byte in text.as_bytes().iter() {
        *counts.entry(*byte).or_default() += 1;
    }
    for byte in unsafe { text.as_bytes_mut() } {
        *byte = if counts[byte] == 1 { b'(' } else { b')' };
    }
    text
}

/// Similar to duplicate_encode_in_place, but using chunks.
/// No noticable difference in performance compared to duplicate_encode_in_place.
fn duplicate_encode_chunks(text: &str) -> String {
    let mut text = text.to_ascii_lowercase();
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for byte in text.as_bytes().iter() {
        *counts.entry(*byte).or_default() += 1;
    }

    unsafe { text.as_bytes_mut() }
        .chunks_mut(CHUNK_SIZE)
        .for_each(|chunk| {
            for byte in chunk {
                *byte = if counts[byte] == 1 { b'(' } else { b')' };
            }
        });
    text
}

/// Similar to duplicate_encode_chunks, but by handling each chunk in parallel
/// using the "rayon" crate.
/// This is significantly faster than the other implementations.
fn duplicate_encode_parallel(text: &str) -> String {
    let mut text = text.to_ascii_lowercase();
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for byte in text.as_bytes().iter() {
        *counts.entry(*byte).or_default() += 1;
    }

    unsafe { text.as_bytes_mut() }
        .par_chunks_mut(CHUNK_SIZE)
        .for_each(|chunk| {
            for byte in chunk {
                *byte = if counts[byte] == 1 { b'(' } else { b')' };
            }
        });
    text
}

/// A different approach to the algorithm that only loops through the input once
/// and builds the output as it goes. By keeping a list of bytes that we have
/// seen before, we can make a decision about which character to add to the
/// output without knowing the exact count of each byte.
/// This implementation is faster than all of the other approaches I have tried
/// above. A more specific description of the algorithm is detailed below.
///
/// Iterate through each char in the string, as bytes.
/// For each byte, If we have already seen the current byte multiple times, we are
/// certain the output will have a ')' at this location. However, if this is our
/// first or second time seeing this byte, further processing is necessary.
///
/// If this is our first time seeing this byte, store it in the "seen_once"
/// HashMap along with its index. If this is the last time we see this byte, the
/// '(' we append to the output will be correct, but if we see it again later,
/// we can use the index to replace this byte with ')'.
///
/// If this is our second time seeing this byte, we need to replace the first
/// occurence with ')', track that we have seen it multiple times, and still
/// append ')' at the end of the loop.
///
/// By the end of the loop, we should end up with a string which has all of the
/// characters correctly encoded, and is the same length as the input.
fn duplicate_encode_track_seen(text: &str) -> String {
    let mut seen_once: HashMap<u8, usize> = HashMap::new();
    let mut seen_multiple: HashSet<u8> = HashSet::new();
    let mut output = String::with_capacity(text.len());

    for (i, byte) in text.as_bytes().iter().enumerate() {
        let byte = byte.to_ascii_lowercase();
        let mut new_char = ')';
        if !seen_multiple.contains(&byte) {
            if let Some(first) = seen_once.remove(&byte) {
                output.replace_range(first..=first, ")");
                seen_multiple.insert(byte);
            } else {
                seen_once.insert(byte, i);
                new_char = '(';
            }
        }
        output.push(new_char);
    }
    output
}

// There are better ways to test in Rust, but...
fn test_functions(functions: &[NamedFunction]) {
    println!("Running tests...");
    let inputs_and_outputs = [
        ("din", "((("),
        ("recede", "()()()"),
        ("Success", ")())())"),
        ("(( @", "))(("),
    ];
    for f in functions.iter().filter(|f| f.enabled) {
        println!("Testing {}...", f.name);
        for (input, expected) in inputs_and_outputs.iter() {
            let actual = (f.body)(input);
            assert_eq!(actual, expected.to_string(), "Wrong output for {}", f.name);
        }
    }
    println!("All tests successfully passed.");
}

fn time_functions(functions: &[NamedFunction]) {
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
    let input_word = input_word_chunk.as_str();
    assert_eq!(input_word.len(), TOTAL_INPUT_SIZE);
    println!(" took {} ms.", start.elapsed().as_millis());

    println!(
        "Timing functions on {} characters per test (over {} tests)",
        CHARS_PER_TEST.to_formatted_string(&Locale::en),
        NUMBER_OF_TEST_RUNS
    );

    // Get the longest function name in the list for formatted printing.
    let longest_name_len = functions.iter().map(|f| f.name.len()).max().unwrap();

    for f in functions.iter().filter(|f| f.enabled) {
        let start = Instant::now();
        for _ in 0..NUMBER_OF_TEST_RUNS {
            (f.body)(input_word);
        }
        let finish = start.elapsed();
        println!(
            "{:<max_len$} | {1:.precision$} sec | {2:.precision$} per (avg)",
            f.name,
            finish.as_secs_f64(),
            finish.as_secs_f64() / NUMBER_OF_TEST_RUNS as f64,
            max_len = longest_name_len,
            precision = OUTPUT_PRECISION
        );
    }

    println!(
        "Took {0:.3} seconds in total.",
        start.elapsed().as_secs_f64()
    );
}

fn main() {
    assert!(CHUNK_SIZE > 0);
    // Using the "enabled" flag, choose which functions to include in the output.
    // With this flag, you won't have to sit through tests you're not interested in.
    let functions = [
        NamedFunction {
            name: "duplicate_encode",
            body: duplicate_encode,
            enabled: true,
        },
        NamedFunction {
            name: "duplicate_encode_default",
            body: duplicate_encode_default,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_capacity",
            body: duplicate_encode_capacity,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_lower",
            body: duplicate_encode_lower,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_counter",
            body: duplicate_encode_counter,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_itertools",
            body: duplicate_encode_itertools,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_map",
            body: duplicate_encode_map,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_bytes",
            body: duplicate_encode_bytes,
            enabled: false,
        },
        NamedFunction {
            name: "duplicate_encode_in_place",
            body: duplicate_encode_in_place,
            enabled: true,
        },
        NamedFunction {
            name: "duplicate_encode_chunks",
            body: duplicate_encode_chunks,
            enabled: true,
        },
        NamedFunction {
            name: "duplicate_encode_parallel",
            body: duplicate_encode_parallel,
            enabled: true,
        },
        NamedFunction {
            name: "duplicate_encode_track_seen",
            body: duplicate_encode_track_seen,
            enabled: true,
        },
    ];
    test_functions(&functions);
    time_functions(&functions);
}
