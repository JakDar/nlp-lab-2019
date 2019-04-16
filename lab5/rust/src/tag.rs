use core::borrow::Borrow;
use core::fmt::Pointer;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::fs::write;
use std::process::exit;
use std::thread;

use std::thread::JoinHandle;
use std::time::Duration;
use itertools::Batching;
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json;

use regex::Regex;
use reqwest;
use reqwest::get;

use crate::preprocess::ls;
use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::ffi::OsString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaggedBigram {
    pub w1: Unigram,
    pub w2: Unigram,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Unigram {
    pub w: String,
    pub cat: String,
}


fn tagging_thread(port: i32, filenames: Vec<String>) -> JoinHandle<HashMap<TaggedBigram, i32>> {
    thread::spawn(move || {
        let mut file_count = 0;
        let files_size = filenames.len();
        println!("spawned thread for port {}", port);
        let mut bigrams: HashMap<TaggedBigram, i32> = HashMap::new();
        filenames.iter().flat_map(|path|
            {
                let r = get_tags(port, path);
                file_count += 1;
                println!("Port {}: {} out of {} finished", port, file_count,files_size);
                r
            })
            .for_each(|bigram| {
                let count = bigrams.get(&bigram).unwrap_or(&0i32);
                bigrams.insert(bigram.clone(), *count);
            });
        println!("Port {} finished", port);
        bigrams
    })
}


fn merged_counters(h1: HashMap<TaggedBigram, i32>, h2: HashMap<TaggedBigram, i32>) -> HashMap<TaggedBigram, i32> {
    let mut res = h1;

    for (bigram, h2_count) in h2 {
        let h1_count = res.get(&bigram).unwrap_or(&0);
        res.insert(bigram, h1_count + h2_count);
    }

    res
}


pub fn test_paralell() {
    let mut dir = ls("../../lower_ustawy");
//    let ports = vec![9200,9201,9202,9203,9204];

    let mut rng = rand::thread_rng();
    let mut hanldes: Vec<JoinHandle<HashMap<TaggedBigram, i32>>> = vec![];


    let mut x1 = dir.into_iter()
        .map(|x| (rng.gen_range(9200i32, 9206i32), x))
//        .take(2)
        .sorted();

    let sorted_by_port: Vec<(i32, OsString)> = x1.collect::<Vec<(i32, OsString)>>();

    let damn = sorted_by_port.into_iter();
    for (port, paths) in &damn.group_by(|(p, os)| *p) {
        hanldes.push(tagging_thread(port, paths.map(|(x, y)| y.to_str().unwrap().to_string()).collect::<Vec<String>>()));
    }


    let bigrams = hanldes.into_iter().map(move |join_handle| join_handle.join().unwrap())
        .fold1(|x, y| merged_counters(x, y))
        .unwrap_or(HashMap::new());

    println!("{:?}", bigrams);


    let bigram_vec = bigrams.into_iter().collect::<Vec<(TaggedBigram, i32)>>();
    let serialized = serde_json::to_string(&bigram_vec).unwrap();
    write("bigrams.out", serialized).unwrap();

    println!("FINISHED!!!!");


//    let deserialized = read_to_string("bigrams.out").unwrap();
//    let file: Vec<(TaggedBigram,i32)> = serde_json::from_str(&deserialized).unwrap();
//
//    assert_eq!(bigram_vec,file);
}


pub fn test() { //279 s for 4
    let dir = ls("../../lower_ustawy");

    let mut bigrams: HashMap<TaggedBigram, i32> = HashMap::new();

    dir.iter().take(4).flat_map(move |ospath|
        get_tags(9200, ospath.to_str().unwrap()))
        .for_each(|bigram| {
            let count = bigrams.get(&bigram).unwrap_or(&0i32);
            bigrams.insert(bigram.clone(), *count);
        });

    println!("{:?}", bigrams);
}

fn get_tags(port: i32, filename: &str) -> Vec<TaggedBigram> {
    let client = reqwest::Client::builder().timeout(Duration::from_secs(70)).build().unwrap();
    let body = read_to_string(filename).unwrap();

    println!("Getting tags for {:?}", filename);
    let mut res = client.post(&format!("http://localhost:{}", port)) //todo:bcm - set timeout
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
    let caps = re.captures(s).unwrap(); //todo:bcm - none here
    Unigram {
        w: caps["lemma"].to_string(),
        cat: caps["cat"].to_string(),
    }
}

#[test]
fn parse_cat_test() {
    assert_eq!(Unigram { w: "lubić".to_string(), cat: "fin".to_string() }, parse_cat("	lubić	fin:sg:ter:imperf	disamb"))
}


