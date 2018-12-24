use std::env;
use std::fs;

enum Preamble {
    Comment(String),
    Import(String),
    Require(String),
}

#[test]
fn it_works() {
    remove_prefix("%abc", "%").unwrap();
    assert!(remove_prefix("%abc", "@").is_none())
}

fn remove_prefix<'a>(input: &'a str, pattern: &str) -> Option<&'a str> {
    if input.starts_with(pattern) {
        let (pat, ans) = input.split_at(pattern.len());
        assert_eq!(pat, pattern);
        Some(ans)
    } else {
        None
    }
}

// preamble can only contain `@import:`, `@require:` and comments (at least, I believe so.)
// the sequence `@import:` and `@require:` must not be interrupted with spaces.
fn split_preamble(contents: &str) -> (Vec<Preamble>, std::iter::Peekable<std::str::Lines<'_>>) {
    let mut lines = contents.lines().peekable();
    let mut preamble_vec = vec![];
    while let Some(line) = lines.peek() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            lines.next();
            continue;
        }

        if trimmed.starts_with("%") {
            let content = trimmed.split_at("%".len()).1.trim_start();
            preamble_vec.push(crate::Preamble::Comment(content.to_string()));
            lines.next();
            continue;
        } else if trimmed.starts_with("@import:") {
            let content = trimmed.split_at("@import:".len()).1.trim_start();
            preamble_vec.push(crate::Preamble::Import(content.to_string()));
            lines.next();
            continue;
        } else if trimmed.starts_with("@require:") {
            let content = trimmed.split_at("@require:".len()).1.trim_start();
            preamble_vec.push(crate::Preamble::Require(content.to_string()));
            lines.next();
            continue;
        }

        // if anything else, the preamble has terminated

        break;
    }

    (preamble_vec, lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    //println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let (preamble, rest) = split_preamble(&contents);

    for item in &preamble {
        match item {
            crate::Preamble::Comment(c) => println!("% {}", c),
            crate::Preamble::Require(c) => println!("@require: {}", c),
            crate::Preamble::Import(c) => println!("@import: {}", c),
        }
    }

    println!();

    for line in rest {
        println!("{}", line)
    }
}
