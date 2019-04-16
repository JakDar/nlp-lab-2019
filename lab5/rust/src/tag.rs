use core::borrow::Borrow;
use core::fmt::Pointer;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::fs::write;
use std::process::exit;
use std::thread;
use crate::utils;
use std::thread::JoinHandle;
use std::time::Duration;
use itertools::Batching;
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json;

use regex::Regex;
use reqwest;
use reqwest::get;

use crate::preprocess::{ls, ls_filenames};
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


fn count_tagged_bigrams(v: Vec<(TaggedBigram)>) -> HashMap<TaggedBigram, i32> {
    let mut bigrams: HashMap<TaggedBigram, i32> = HashMap::new();
    v.into_iter().for_each(|bigram| {
        let count = bigrams.get(&bigram).unwrap_or(&0i32);
        bigrams.insert(bigram.clone(), *count + 1);
    });

    //filter only once occuring
    bigrams.into_iter().filter(|(x,y)| *y>1).collect::<HashMap<TaggedBigram,i32>>()
}

fn tagging_thread(port: i32, filenames: Vec<String>) -> JoinHandle<HashMap<TaggedBigram, i32>> {
    thread::spawn(move || {
        let mut file_count = 0;
        let files_size = filenames.len();
        println!("spawned thread for port {}", port);
        let bigrams =
            filenames.iter().map(|path|
                {
                    let r = count_tagged_bigrams(get_tags(port, &format!("../../lower_ustawy/{}", path)));

                    let for_saving = utils::hash_to_vec(r.clone());

                    let json = serde_json::to_string(&for_saving).unwrap();
                    write(format!("../../lemma_bigram_ustawy/{}", path), json).unwrap();
                    file_count += 1;
                    println!("Port {}: {} out of {} finished", port, file_count, files_size);

                    r
                })
                .fold1(|h1, h2| merged_counters(h1, h2)).unwrap_or(HashMap::new());
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

fn load_serialized_counter() -> HashMap<TaggedBigram, i32> {
    let serializedd_dir = ls("../../lemma_bigram_ustawy");

    serializedd_dir.iter().map(|filename|
        {
            let json = read_to_string(filename).unwrap();
            let list: Vec<(TaggedBigram, i32)> = serde_json::from_str(&json).unwrap();
            utils::vec_to_hash(list)
        }
    )
        .fold1(|x, y| merged_counters(x, y))
        .unwrap_or(HashMap::new())
}


pub fn test_paralell() {
    let mut source_dir = ls_filenames("../../lower_ustawy");
    let mut dst_dir = ls_filenames("../../lemma_bigram_ustawy").into_iter().collect::<HashSet<String>>();

    println!("Docs to hanlde: {} of {}", source_dir.len() - dst_dir.len(), source_dir.len());

    let already_serialized = load_serialized_counter();

    let mut rng = rand::thread_rng();
    let mut hanldes: Vec<JoinHandle<HashMap<TaggedBigram, i32>>> = vec![];


    let mut x1 = source_dir.into_iter()
        .filter(|path| !dst_dir.contains(path))
        .map(|x| (rng.gen_range(9200i32, 9206i32), x))
//        .take(2)
        .sorted();

    let sorted_by_port: Vec<(i32, String)> = x1.collect::<Vec<(i32, String)>>();

    let damn = sorted_by_port.into_iter();
    for (port, paths) in &damn.group_by(|(p, os)| *p) {
        hanldes.push(tagging_thread(port, paths.map(|(x, y)| y).collect::<Vec<String>>()));
    }


    let bigrams = hanldes.into_iter().map(move |join_handle| join_handle.join().unwrap())
        .fold(already_serialized, |x, y| merged_counters(x, y));

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
        get_tags(9200, ospath))
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
            let l1 = lines.get(2 * i).unwrap();
            let opt_cat = parse_cat(l1, lines.get(2 * i + 1).unwrap())
                .filter(|cat| cat.w.chars().all(|c| c.is_alphabetic()));
            if let Some(cat) = opt_cat {
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


fn parse_cat(label: &str, s: &str) -> Option<Unigram> {
    let re = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    match re.captures(s) {
        Option::Some(caps) =>
            Some(Unigram {
                w: caps["lemma"].to_string(),
                cat: caps["cat"].to_string(),
            }),
        Option::None => {
            println!("Cannot parse categor from  '{}', label: {}", s, label);
            Option::None
        }
    }
}

#[test]
fn parse_cat_test() {
    assert_eq!(Option::Some(Unigram { w: "lubić".to_string(), cat: "fin".to_string() }), parse_cat("", "	lubić	fin:sg:ter:imperf	disamb"));
    assert_eq!(Option::None, parse_cat("", "	interp	disamb'"))
}


