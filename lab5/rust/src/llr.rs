use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

use std::collections::HashMap;
use crate::tag::{TaggedBigram, Unigram};
use std::fs::read_to_string;

#[derive(Debug)]
struct LlrEntry {
    w1: Unigram,
    w2: Unigram,
    llr: f64,
}

impl Display for LlrEntry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "LlrEntry(w1: \"{:?}\",\tw2: \"{:?}\",\tllr: {:.*}", self.w1, self.w2, 4, self.llr)
    }
}

//def llr_2x2(k11, k12, k21, k22):
//'''Special case of llr with a 2x2 table'''
//return 2 * (denormEntropy([k11+k12, k21+k22]) +
//denormEntropy([k11+k21, k12+k22]) -
//denormEntropy([k11, k12, k21, k22]))


fn llr2x2(k11: f64, k12: f64, k21: f64, k22: f64) -> f64 {
    2f64 * (
        denorm_entropy(vec![k11 + k12, k21 + k22]) +
            denorm_entropy(vec![k11 + k21, k12 + k22]) -
            denorm_entropy(vec![k11, k12, k21, k22])
    )
}


//def denormEntropy(counts):
//'''Computes the entropy of a list of counts scaled by the sum of the counts. If the inputs sum to one, this is just the normal definition of entropy'''
//counts = list(counts)
//total = float(sum(counts))
//# Note tricky way to avoid 0*log(0)
//return -sum([k * math.log(k/total + (k==0)) for k in counts])

fn denorm_entropy(counts: Vec<f64>) -> f64 {
    let total = counts.iter().sum::<f64>();

    let sum = counts.iter().map(|k| {
        if *k == 0f64 {
            *k
        } else {
            k * f64::ln(*k / total)
        }
    }).sum::<f64>();

    sum * -1f64
}

pub fn count_words(entries: Vec<(TaggedBigram, i32)>) -> (HashMap<Unigram, i64>, HashMap<Unigram, i64>) {
    let mut hmap1: HashMap<Unigram, i64> = HashMap::new();
    let mut hmap2: HashMap<Unigram, i64> = HashMap::new();


    for (entry, count) in entries {
        let new_val = hmap1.get(&entry.w1).unwrap_or(&0i64) + count as i64;
        hmap1.insert(entry.w1, new_val);

        let new_val2 = hmap2.get(&entry.w2).unwrap_or(&0i64) + count as i64;
        hmap2.insert(entry.w2, new_val2);
    }
    (hmap1, hmap2)
}

pub fn llr() {
    let json = read_to_string("bigrams.out").unwrap();
    let bigrams: Vec<(TaggedBigram, i32)> = serde_json::from_str(&json).unwrap();
    let (w1_counts, w2_counts) = count_words(bigrams.clone());
    let bigram_count = bigrams.iter().map(|(b, count)| (*count) as i64).sum::<i64>() as f64;

    let bigrams_iter = bigrams.iter();


    let mut llrs = bigrams_iter.map(|(entry, count)| {
        let k11 = (*count) as f64;
        let w1_count = *(w1_counts.get(&entry.w1).unwrap_or(&0i64)) as f64;
        let w2_count = *(w2_counts.get(&entry.w2).unwrap_or(&0i64)) as f64;

        let k12 = w1_count - k11;
        let k21 = w2_count - k11;
        let k22 = bigram_count as f64 - (k11 + k12 + k21);


        let llr = llr2x2(k11, k12, k21, k22);
        if llr.partial_cmp(&1f64) == None {
            println!("OH no, NANs!");
            println!("k11:{}, k12:{}, k21:{}, k22:{}, llr: {}", k11, k12, k21, k22, llr);
        }

        LlrEntry { w1: entry.w1.clone(), w2: entry.w2.clone(), llr }
    }).collect::<Vec<LlrEntry>>();


    llrs.sort_by(|a, b| b.llr.partial_cmp(&a.llr).unwrap_or(Ordering::Equal));
//    llrs.reverse();

    llrs.iter()
        // .filter(|entry| entry.w1.cat == "subst" && entry.w2.cat == "adj")
        .take(50).for_each(|c| println!("{}", c));
}


