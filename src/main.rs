use std::{env, fs};


fn get_file(path: &str) -> std::io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;

    let trimmed_content = content.trim();
    let inner = trimmed_content.strip_suffix("]").and_then(|s| s.strip_prefix("[")).unwrap_or(trimmed_content);

    let mut vec: Vec<String>= inner
        .split('\n')
        .map(|s| s.trim())
        .map(|s| s.trim_matches(','))
        .map(|s| s.trim_matches('"'))
        .map(|s| s.to_string())
        .collect();

    if vec.last().is_some_and(|s| s.is_empty()) {
        vec.pop();
    }
    if vec.first().is_some_and(|s| s.is_empty()) {
        vec.remove(0);
    }

    Ok(vec)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Wrong amt of input arguments! Only provide the Path to the File!");
        std::process::exit(1);
    }

    let objects = match get_file(&args[1]) {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Error Parsing the File: {e}");
            std::process::exit(1);
        },
    };

    let mut panagram_counter = PanagramCounter::default();

    for s in objects {
        match is_kind_of_panagram(s) {
            KindOfPanagram::None => panagram_counter.none += 1,
            KindOfPanagram::ImperfectPanagram => panagram_counter.imperfect += 1,
            KindOfPanagram::Perfect => panagram_counter.perfect += 1,
        }
    }

    println!("Result of Panagram Counter by Cerdelen");
    println!("");
    println!("Amount of non Panagrams:        {:>3}", panagram_counter.none);
    println!("Amount of imperfect Panagrams:  {:>3}", panagram_counter.imperfect);
    println!("Amount of perfect Panagrams:    {:>3}", panagram_counter.perfect);
}

enum KindOfPanagram {
    None,
    Perfect,
    ImperfectPanagram
}

#[derive(Debug, Default)]
struct PanagramCounter {
    none: u16,
    perfect: u16,
    imperfect: u16,
}

// Time Complexity is O(n) with n == size of String.
// Space Complexity is O(1). Constant Space complexity.
fn is_kind_of_panagram(s: String) -> KindOfPanagram {
    // early return statement
    if s.len() < 26 { return KindOfPanagram::None }

    let mut char_map: [bool; 26] = [false; 26];
    let mut unique_chars = 0;
    let mut out = KindOfPanagram::Perfect;

    for char in s.chars() {
        let lower_case_ascii_char = char.to_ascii_lowercase();

        if lower_case_ascii_char.is_alphabetic() {
            let idx = (lower_case_ascii_char as u8 - b'a') as usize;
            if !char_map[idx] {
                unique_chars += 1;
                char_map[idx] = true;
            } else {
                out = KindOfPanagram::ImperfectPanagram;
                // early return statement to avoid Long strings
                if unique_chars == 26 {
                    return out;
                }
            }
        }
    }

    if unique_chars != 26 {
        return KindOfPanagram::None;
    }

    out
}
