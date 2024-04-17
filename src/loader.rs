use std::fs::File;
use bitvec::prelude::BitVec;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Word {
    pub word: String,
    pub freq: u64,
    pub rank: u64,
}

pub fn load_words(path: &str, max_lines: u64) -> anyhow::Result<Vec<Word>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut words = Vec::new();
    let mut rank = 0;
    while let Ok(_) = reader.read_line(&mut buf) {
        if buf.is_empty() { break; }
        words.push(line_to_word(buf.clone(), rank)?);
        rank += 1;
        buf.clear();
        if rank > max_lines { break; }
    }
    Ok(words)
}

pub fn line_to_word(input: String, rank: u64) -> anyhow::Result<Word> {
    let mut line = input.clone().replace("\n", "").replace("\r", "");
    let mut wf = line.split(",");
    let word = wf.next().unwrap().to_string();
    let freq = wf.next().unwrap().parse().expect(&format!("{input}"));
    Ok(Word { word, freq, rank })
}

pub fn load_codes(path: &str, max_lines: u64) -> anyhow::Result<HashMap<String, BitVec<u8>>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut hashmap = HashMap::new();

    let mut rank = 0;
    while let Ok(_) = reader.read_line(&mut buf) {
        if buf.is_empty() { break; }
        let comma_idx = buf.find(",").unwrap();
        let (word, code) = buf.split_at(comma_idx);
        hashmap.insert(word.to_string(), bitvec_from_str(&code));    

        rank += 1;
        buf.clear();
    }

    Ok(hashmap)
}

pub fn bitvec_from_str(s: &str) -> BitVec<u8> {
    let mut v = BitVec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '1' => v.push(true),
            '0' => v.push(false),
            _ => {}
        }
    }
    v
}
