use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use serde;
use serde_pickle;

use crate::load_bigrams;
use crate::load_bigrams::Entry;

#[derive(Debug, Clone)]
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


pub fn count_words(entries: Vec<Entry>) -> (HashMap<String, i64>,HashMap<String, i64>) {
    let mut hmap1: HashMap<String, i64> = HashMap::new();
    let mut hmap2: HashMap<String, i64> = HashMap::new();


    for entry in entries {
        let new_val = hmap1.get(&entry.w1).unwrap_or(&0i64) + entry.count;
        hmap1.insert(entry.w1, new_val);

        let new_val2 = hmap2.get(&entry.w2).unwrap_or(&0i64) + entry.count;
        hmap2.insert(entry.w2, new_val2);
    }
    (hmap1,hmap2)
}


pub fn pmis() {
    let entries_vec = load_bigrams::load_entries();
    let entries = entries_vec.iter();
    let bigram_count = entries_vec.iter().map(|x| x.count).sum::<i64>();

    let (w1_counts,w2_counts)= count_words(entries_vec.clone());

    let mut pmis = entries.map(|entry| {
        let w1_count = *(w1_counts.get(entry.w1.as_str()).unwrap_or(&0i64)) as f64;
        let w2_count = *(w2_counts.get(entry.w2.as_str()).unwrap_or(&0i64)) as f64;


        let pmi = ((entry.count * bigram_count) as f64 / (w1_count * w2_count)).log(2.0);

        let res = PmiEntry { w1: entry.w1.clone(), w2: entry.w2.clone(), pmi };
        if pmi as i32 == 18 {
            println!("{};   w1: {}, w2: {}, bigram:{} entry: {}", res.clone(), w1_count, w2_count, bigram_count, entry.count);
        }

        res
    }).collect::<Vec<PmiEntry>>();


    pmis.sort_by(|a, b| b.pmi.partial_cmp(&a.pmi).unwrap());

    pmis.iter().take(30).for_each(|c| println!("{}", c));


//    for (i, entry) in pmis.iter().enumerate() {
//        if i % 100 == 0 {
//            println!("{}, {}", i, entry);
//        }
//    }
}