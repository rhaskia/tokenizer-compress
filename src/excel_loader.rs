use calamine::{RangeDeserializerBuilder, Data, RangeDeserializer};
use calamine::{open_workbook, Reader, Xlsx};
use serde::Deserialize;
use std::io::BufReader;
use std::fs::File;

pub struct ExcelLoader {
    excel: Xlsx<BufReader<File>>
}

impl ExcelLoader {
    pub fn new() -> Self {
        Self { excel: open_workbook("./wordFrequency.xlsx").unwrap() }
    }

    pub fn load_words(&mut self) -> Result<Vec<Word>, anyhow::Error> {
        let word_forms_sheet =
            self.excel.worksheet_range("4 forms (219k)").expect("Could not find forms range");

        let range: RangeDeserializer<'_, Data, Word> = RangeDeserializerBuilder::with_headers(&[
            "rank", "word", "freq", "#texts", "%caps",
        ])
        .from_range(&word_forms_sheet)?;

        Ok(range.map(|w| w.unwrap()).collect::<Vec<Word>>())
    }

    pub fn load_word_forms(&mut self) -> Result<Vec<WordForm>, anyhow::Error> {
        let word_forms_sheet =
            self.excel.worksheet_range("3 wordForms").expect("Could not find wordforms range");

        let range: RangeDeserializer<'_, Data, WordForm> = RangeDeserializerBuilder::with_headers(&[
            "lemRank", "lemma", "PoS", "lemFreq", "wordFreq", "word",
        ])
        .from_range(&word_forms_sheet)?;

        Ok(range.map(|w| w.unwrap()).collect::<Vec<WordForm>>())
    }
}



#[derive(Deserialize, Debug)]
pub enum PartOfSpeech {
    #[serde(rename = "n")]
    Noun,
    #[serde(rename = "v")]
    Verb,
    #[serde(untagged)]
    Other(String),
}

pub struct Lemma {
    rank: u16,
    lemma: String,
    part: PartOfSpeech,
    freq: u32,
    per_mil: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WordForm {
    pub lem_rank: u16,
    pub lemma: String,
    #[serde(rename = "PoS")]
    pub pos: PartOfSpeech,
    pub lem_freq: u32,
    pub word_freq: u32,
    pub word: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub rank: u16,
    pub word: String,
    pub freq: u32,
    #[serde(rename = "#texts")]
    pub texts: u32,
    #[serde(rename = "%caps")]
    pub caps: f32,
    //genre_freq: Vec<u32>, 
}
