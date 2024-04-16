use std::fs::File;
use std::io::{BufRead, BufReader};

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
