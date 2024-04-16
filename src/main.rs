#![feature(string_remove_matches)]
mod loader;
use loader::Word;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum CWord {
    Word(u64),
    Newline,
    Punctuation(char),
    Number(char),
    Unknown(String),
}

impl CWord {
    pub fn to_bits(&self) -> Vec<bool> {
        match self {
            CWord::Word(w) => todo!(),
            CWord::Newline => vec![true],
            CWord::Punctuation(p) => todo!(),
            CWord::Number(n) => todo!(),
            CWord::Unknown(u) => todo!(),
        } 
    }
}

pub fn split_words(s: String, words: &Vec<Word>) -> Vec<CWord> {
    let mut compressed = Vec::new();
    let mut word = String::new();

    for c in s.chars() {
        if !c.is_alphabetic() {
            if !word.is_empty() {
                compressed.push(match_word(word, &words));
                word = String::new();
            }
        } else {
            word.push(c);
        }

        if c == '\n' {
            compressed.push(CWord::Newline);
        }

        if c.is_numeric() {
            compressed.push(CWord::Number(c));
        }

        if c.is_ascii_punctuation() {
            compressed.push(CWord::Punctuation(c))
        }

        // TODO: other possible characters
    }

    compressed
}

pub fn match_word(s: String, words: &Vec<Word>) -> CWord {
    for word in words {
        if word.word.to_lowercase() == s.to_lowercase() {
            return CWord::Word(word.rank);
        }
    }
    CWord::Unknown(s.to_lowercase())
}

fn count_cwords(cwords: &Vec<CWord>) -> (i32, i32, i32, i32, i32) {
    let mut counts = (0, 0, 0, 0, 0);
    for cword in cwords {
        match cword {
            CWord::Word(_) => counts.0 += 1,
            CWord::Newline => counts.1 += 1,
            CWord::Punctuation(_) => counts.2 += 1,
            CWord::Number(_) => counts.3 += 1,
            CWord::Unknown(_) => counts.4 += 1,
        }
    }
    counts
}



fn main() -> anyhow::Result<()> {
    let words = loader::load_words("unigram_freq.csv", 5_000)?;
    println!("loaded word list");

    let data_set = std::fs::read_to_string("pg11.txt")?;
    let split = split_words(data_set, &words);
    println!("{:?}", count_cwords(&split));



    Ok(())
}
