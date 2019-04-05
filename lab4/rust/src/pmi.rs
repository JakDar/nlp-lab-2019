use std::fmt::{Display, Error, Formatter};
use serde;
use serde_pickle;

use crate::load_bigrams;
use crate::load_bigrams::Entry;
use std::collections::HashMap;


#[derive(Debug)]
struct PmiEntry {
    w1: String,
    w2: String,
    pmi: f64,
}

impl Display for PmiEntry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "PmiEntry(w1: \"{}\",\tw2: \"{}\",\tpmi: {:.*}", self.w1, self.w2, 4, self.pmi)
    }
}


pub fn count_words(entries:Vec<Entry>) -> HashMap<String,i64>{
    let mut hmap: HashMap<String,i64> = HashMap::new();


    for entry in entries { //todo:bcm - here we should have entry count
        let new_val = hmap.get(&entry.w1).unwrap_or(&0i64) +entry.count;
        hmap.insert(entry.w1,new_val);

        let new_val2 = hmap.get(&entry.w2).unwrap_or(&0i64) + entry.count;
        hmap.insert(entry.w2,new_val2);
    }
    hmap
}


pub fn pmis() {

    //todo- why heap allocated box?

    let entries_vec = load_bigrams::load_entries();
    let entries = entries_vec.iter();
    let bigram_count = entries_vec.iter().map(|x|x.count).sum::<i64>();

    let word_counter = count_words(entries_vec.clone());

    let mut pmis = entries.map(|entry| {
        let w1_count = *(word_counter.get(entry.w1.as_str()).unwrap_or(&0i64)) as f64;
        let w2_count = *(word_counter.get(entry.w2.as_str()).unwrap_or(&0i64)) as f64;

        //todo:review - compare results with lukasz
        let pmi = ((entry.count * bigram_count)as f64 / (w1_count * w2_count)).log(2.0);

        PmiEntry { w1: entry.w1.clone(), w2: entry.w2.clone(), pmi }
    }).collect::<Vec<PmiEntry>>();


    pmis.sort_by(|a, b| b.pmi.partial_cmp(&a.pmi).unwrap());

    pmis.iter().take(30).for_each(|c| println!("{}", c));
}