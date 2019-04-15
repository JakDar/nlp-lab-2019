use core::borrow::Borrow;
use core::fmt::Pointer;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::process::exit;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use regex::Regex;
use reqwest;
use reqwest::get;

use crate::preprocess::ls;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaggedBigram {
    pub w1: Unigram,
    pub w2: Unigram,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]  // could be optmized by using enums as categories
pub struct Unigram {
    pub w: String,
    pub cat: String,
}


fn tagging_thread(port: i32, filenames: Vec<String>) -> JoinHandle<HashMap<TaggedBigram, i32>> {
    thread::spawn(move || {
        let mut bigrams: HashMap<TaggedBigram, i32> = HashMap::new();
        filenames.iter().flat_map(|path|
            get_tags(port, path))
            .for_each(|bigram| {
                let count = bigrams.get(&bigram).unwrap_or(&0i32);
                bigrams.insert(bigram.clone(), *count);
            });
        bigrams
    })
}


fn merge_counters(h1:HashMap<TaggedBigram,i32>,w2:HashMap<TaggedBigram,i32>){


}

pub fn test() {

    let x= tagging_thread(9200,vec![]);

    exit(1);
    let dir = ls("../../lower_ustawy");
    let head = dir.first().unwrap().to_str().unwrap();
    println!("{}", head);


    let mut bigrams: HashMap<TaggedBigram, i32> = HashMap::new();

    let x = dir.iter().take(10).flat_map(move |ospath|
        get_tags(9200, ospath.to_str().unwrap()))
        .for_each(|bigram| {
            let count = bigrams.get(&bigram).unwrap_or(&0i32);
            bigrams.insert(bigram.clone(), *count);
        });

    println!("{:?}", bigrams);

    exit(1);
}

fn get_tags(port: i32, filename: &str) -> Vec<TaggedBigram> {
    let client = reqwest::Client::new();
    let body = read_to_string(filename).unwrap();

    println!("Getting tags for {:?}", filename);
    let mut res = client.post(&format!("http://localhost:{}", port))
        .body(body)
        .send().unwrap();

    let body = res.text().unwrap();
    parse(body)
}


fn parse(input: String) -> Vec<TaggedBigram> {
    let with_empty: Vec<&str> = input.split("\n").collect();
    let lines = with_empty.iter().map(|x| *x).filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    if lines.len() % 2 != 0 {
        println!("{:?}", lines);

        panic!("Tag response lines should be even");
    } else {
        let mut bigrams: Vec<TaggedBigram> = vec![];
        let mut prev_unigram: Option<Unigram> = Option::None;
        for i in 0..lines.len() / 2 {
//            println!("{}", lines.get(2 * i).unwrap());
//            println!("{}", lines.get(2 * i + 1).unwrap());
            let l1: Vec<&str> = lines.get(2 * i).unwrap().split("\t").collect();
            let cat = parse_cat(lines.get(2 * i + 1).unwrap());
            let mut chars = cat.w.chars();
            if chars.all(|c| c.is_alphabetic()) {
                if let Some(unigram) = prev_unigram {
                    let bigram = TaggedBigram { w1: unigram, w2: cat.clone() };
//                    println!("{:?}", bigram.clone());
                    bigrams.push(bigram);
                }

                prev_unigram = Some(cat);
            } else {
                prev_unigram = None
            }
//            println!("---------");
        }
        bigrams
    }
}


fn parse_cat(s: &str) -> Unigram {
    let re = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    let caps = re.captures(s).unwrap();
    Unigram {
        w: caps["lemma"].to_string(),
        cat: caps["cat"].to_string(),
    }
}

#[test]
fn parse_cat_test() {
    assert_eq!(Unigram { w: "lubić".to_string(), cat: "fin".to_string() }, parse_cat("	lubić	fin:sg:ter:imperf	disamb"))
}


