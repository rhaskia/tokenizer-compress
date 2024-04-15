mod excel_loader;
use excel_loader::Word;

pub enum CompressedWord {
    Word(u16),
    Unknown(String),
    Newline,
    Punctuation(String),
}

pub fn split_words(s: String) -> Vec<String> {
    let simple_split = s.split(|c: char| c.is_whitespace());
    simple_split.map(|spl| spl.to_string()).collect()
}

pub fn clean_words(s: String, word_list: Vec<Word>) -> Vec<CompressedWord> {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
    let split = lines.iter().map(|line| split_words(line.to_string())).collect::<Vec<Vec<String>>>();
    println!("{split:?}");
    panic!();
}

fn main() -> anyhow::Result<()> {
    let mut loader = excel_loader::ExcelLoader::new();
    let word_forms = loader.load_word_forms();
    let words = loader.load_words();

    let data_set = std::fs::read_to_string("pg11.txt")?;
    let split = clean_words(data_set, words?);

    Ok(())
}

