use std::fs;

#[macro_use]
extern crate clap;

enum Preamble {
    Comment(String),
    Import(String),
    Require(String),
}

#[test]
fn it_works() {
    if let crate::Foo::Removed(_s) = remove_prefix("%abc", "%") {
    } else {
        panic!()
    }
    if let crate::Foo::NotRemoved(_s) = remove_prefix("%abc", "@") {
    } else {
        panic!()
    }
}

enum Foo<'a> {
    Removed(&'a str),
    NotRemoved(&'a str),
}

fn remove_prefix<'a>(input: &'a str, pattern: &str) -> Foo<'a> {
    if input.starts_with(pattern) {
        let (pat, ans) = input.split_at(pattern.len());
        debug_assert_eq!(pat, pattern);
        crate::Foo::Removed(ans)
    } else {
        crate::Foo::NotRemoved(input)
    }
}

// preamble can only contain `@import:`, `@require:` and comments (at least, I believe so.)
// the sequence `@import:` and `@require:` must not be interrupted with spaces.
fn split_preamble(contents: &str) -> (Vec<Preamble>, std::iter::Peekable<std::str::Lines<'_>>) {
    let mut lines = contents.lines().peekable();
    let mut preamble_vec = vec![];
    while let Some(line) = lines.peek() {
        let mut trimmed = line.trim_start();
        if trimmed.is_empty() {
            lines.next();
            continue;
        }

        match remove_prefix(trimmed, "%") {
            crate::Foo::Removed(c) => {
                let content = c.trim_start();
                preamble_vec.push(crate::Preamble::Comment(content.to_string()));
                lines.next();
                continue;
            }
            crate::Foo::NotRemoved(l) => {
                trimmed = l;
            }
        }

        match remove_prefix(trimmed, "@import:") {
            crate::Foo::Removed(c) => {
                let content = c.trim_start_matches(' ');
                // only U+0020s that start the file name are allowed after the colon
                preamble_vec.push(crate::Preamble::Import(content.to_string()));
                lines.next();
                continue;
            }
            crate::Foo::NotRemoved(l) => {
                trimmed = l;
            }
        }

        if let crate::Foo::Removed(c) = remove_prefix(trimmed, "@require:") {
            let content = c.trim_start_matches(' ');
            // only U+0020s that start the file name are allowed after the colon
            preamble_vec.push(crate::Preamble::Require(content.to_string()));
            lines.next();
            continue;
        }

        // if anything else, the preamble has terminated

        break;
    }

    (preamble_vec, lines)
}

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", filename);

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
