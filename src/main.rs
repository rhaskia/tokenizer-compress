use calamine::{Reader, Xlsx, open_workbook};
use calamine::RangeDeserializerBuilder;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum PartOfSpeech {
    #[serde(rename = "n")] 
    Noun,
    #[serde(rename = "v")] 
    Verb,
    #[serde(untagged)]
    Other(String)
}

struct Lemma {
    rank: u16,
    lemma: String,
    part: PartOfSpeech,
    freq: u32,
    per_mil: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WordForm {
   lem_rank: u16,
   lemma: String,
   #[serde(rename = "PoS")] 
   pos: PartOfSpeech,
   lem_freq: u32,
   word_freq: u32,
   word: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook("./wordFrequency.xlsx").unwrap();
    println!("{:?}", excel.sheet_names());
    let word_forms_sheet = excel.worksheet_range("3 wordForms").expect("Could not find wordforms range");

    let iter_records =
            RangeDeserializerBuilder::with_headers(&["lemRank", "lemma", "PoS", "lemFreq", "wordFreq", "word"]).from_range(&word_forms_sheet)?;

    for result in iter_records {
        let record: WordForm = result?;
        println!("{record:?}");
    }

    // 1 lemmas
    // 2 subgenres
    // 3 wordForms
    // 4 forms (219k)
    if let Ok(r) = excel.worksheet_range("3 wordForms") {
        for row in r.rows() {
            println!("row={:?}", row);
        }
    }

    Ok(())
}
