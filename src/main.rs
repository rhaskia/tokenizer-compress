#![feature(string_remove_matches)]
mod loader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use bitvec::{bitvec, vec::BitVec};
use loader::Word;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum CWord {
    Word(Word),
    Newline,
    Punctuation(char),
    Number(u8),
    Unknown(String),
}

pub fn punc_index(p: char) -> u8 {
    match p {
        ' '..='@' => (p as u8) - 32,       // 0 - 32
        '['..='`' => (p as u8) - 91 + 33,  // 33 - 39
        '{'..='~' => (p as u8) - 123 + 39, // 40 - 44
        _ => panic!("char not ascii punctuation"),
    }
}

pub fn index_to_punc(p: u8) -> char {
    match p {
        0..=32 => (p + 32) as char,        // 0 - 32
        33..=39 => (p + 91 - 33) as char,  // 33 - 39
        40..=44 => (p + 123 - 40) as char, // 40 - 44
        _ => panic!("u8 not ascii punctuation"),
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
            compressed.push(CWord::Number(c.to_digit(10).unwrap() as u8));
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
            return CWord::Word(word.clone());
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

// fn decompress(filename: &str) -> anyhow::Result<Vec<CWord>> {
//     let mut escaped = false;
//     let mut cwords = Vec::new();
//     let mut word = String::new();
//
//     let file = std::fs::read(filename)?;
//
//     let mut i = 0;
//     while i + 1 < file.len() {
//         let first_byte = file[i] as u8;
//         let second_byte = file[i + 1] as u8;
//
//         if escaped {
//             if first_byte == 0 && second_byte == 0 {
//                 escaped = false;
//                 cwords.push(CWord::Unknown(word.clone()));
//                 word.clear();
//                 i += 2;
//             } else {
//                 word.push(first_byte as char);
//                 i += 1;
//             }
//         } else {
//             let value = ((first_byte as u16) << 8) | second_byte as u16;
//             if value == 0 {
//                 escaped = true;
//             } else {
//                 match value {
//                     65492..=65534 => {
//                         cwords.push(CWord::Punctuation(index_to_punc(254 - second_byte)))
//                     }
//                     65481..=65491 => cwords.push(CWord::Number(254 - second_byte - 44)),
//                     u16::MAX => cwords.push(CWord::Newline),
//                     _ => cwords.push(CWord::Word(value - 1)),
//                 };
//             }
//             i += 2;
//         }
//     }
//
//     Ok(cwords)
// }

fn main() -> anyhow::Result<()> {
    let words = loader::load_words("unigram_freq.csv", 65536)?;
    println!("loaded word list");

    let h_codes = loader::load_codes("./huffman/huffman_codes.csv", u16::MAX.into())?;
    let data_set = std::fs::read_to_string("./cantrbry/asyoulik.txt")?;
    let split = split_words(data_set, &words);
    println!("{:?}", count_cwords(&split));

    let mut file = std::fs::File::create("compress.tzp")?;

    let mut bits = BitVec::<u8>::new();
    for word in split {
        let mut w_bits = match word {
            CWord::Word(word) => &h_codes[&word.word],
            CWord::Newline => &h_codes["!"],
            CWord::Punctuation(_) => &h_codes["!"],
            CWord::Number(_) => &h_codes["!"],
            CWord::Unknown(_) => &h_codes["!"],
        }.clone();
        bits.append(&mut w_bits);
    }
    file.write_all(bits.as_raw_slice())?;

    // let decompressed = decompress("compress.tzp")?;
    // for (w, w2) in decompressed.iter().zip(split.iter()) {
    //     assert_eq!(w, w2);
    // }

    Ok(())
}
