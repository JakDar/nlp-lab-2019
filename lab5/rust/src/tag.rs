use crate::utils;
use core::borrow::Borrow;
use core::fmt::Pointer;
use itertools::Batching;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::fs::write;
use std::process::exit;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use regex::Regex;
use reqwest;
use reqwest::get;

use crate::preprocess::{ls, ls_filenames};
use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::ffi::OsString;
//todo- fix hardcoded filenames

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
    bigrams
        .into_iter()
        .filter(|(x, y)| *y > 1)
        .collect::<HashMap<TaggedBigram, i32>>()
}

fn tagging_thread(port: i32, filenames: Vec<String>,cat_regex:Regex) -> JoinHandle<HashMap<TaggedBigram, i32>> {
    thread::spawn(move || {
        let mut file_count = 0;
        let files_size = filenames.len();
        println!("spawned thread for port {}", port);
        let bigrams = filenames
            .iter()
            .map(|path| {
                let r =
                    count_tagged_bigrams(get_tags_recovering(port, &format!("../../bench/{}", path), &cat_regex));

                let for_saving = utils::hash_to_vec(r.clone());

                let json = serde_json::to_string(&for_saving).unwrap();
                write(format!("../bench_out/{}", path), json).unwrap();
                file_count += 1;
                println!(
                    "Port {}: {} out of {} finished",
                    port, file_count, files_size
                );

                r
            })
            .fold1(|h1, h2| merged_counters(h1, h2))
            .unwrap_or(HashMap::new());
        println!("Port {} finished", port);
        bigrams
    })
}

fn merged_counters(
    h1: HashMap<TaggedBigram, i32>,
    h2: HashMap<TaggedBigram, i32>,
) -> HashMap<TaggedBigram, i32> {
    let mut res = h1;

    for (bigram, h2_count) in h2 {
        let h1_count = res.get(&bigram).unwrap_or(&0);
        res.insert(bigram, h1_count + h2_count);
    }

    res
}

fn load_serialized_counter(dst_path:&str) -> HashMap<TaggedBigram, i32> {
    let serializedd_dir = ls(dst_path);

    serializedd_dir
        .iter()
        .map(|filename| {
            let json = read_to_string(filename).unwrap();
            let list: Vec<(TaggedBigram, i32)> = serde_json::from_str(&json).unwrap();
            utils::vec_to_hash(list)
        })
        .fold1(|x, y| merged_counters(x, y))
        .unwrap_or(HashMap::new())
}

pub fn test_paralell() {
    let source_path = "../bench";
    let dst_path = "../bench_out";

    let mut source_dir = ls_filenames(source_path);
    let mut dst_dir = ls_filenames(dst_path)
        .into_iter()
        .collect::<HashSet<String>>();

    println!(
        "Docs to hanlde: {} of {}",
        source_dir.len() - dst_dir.len(),
        source_dir.len()
    );

    let already_serialized = load_serialized_counter(dst_path);
//
    let mut rng = rand::thread_rng();
    let mut handles: Vec<JoinHandle<HashMap<TaggedBigram, i32>>> = vec![];

    let mut x1 = source_dir.into_iter()
        .filter(|path| !dst_dir.contains(path))
        .map(|x| (rng.gen_range(9200i32, 9204i32), x))
        .sorted();

    let sorted_by_port: Vec<(i32, String)> = x1.collect::<Vec<(i32, String)>>();
//
    let damn = sorted_by_port.into_iter();
    let cat_regex = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    for (port, paths) in &damn.group_by(|(p, os)| *p) {
        handles.push(tagging_thread(port, paths.map(|(x, y)| y).collect::<Vec<String>>(),cat_regex.clone()));
    }
//
    let bigrams = handles.into_iter().map(move |join_handle| join_handle.join().unwrap())
        .fold(already_serialized, |x, y| merged_counters(x, y));
//
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

pub fn test() {
    //279 s for 4
    let dir = ls("../bench");

    let cat_regex = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    let mut bigrams: HashMap<TaggedBigram, i32> = HashMap::new();

    dir.iter()
        .take(4)
        .flat_map(move |ospath| get_tags_recovering(9200, ospath, &cat_regex))
        .for_each(|bigram| {
            let count = bigrams.get(&bigram).unwrap_or(&0i32);
            bigrams.insert(bigram.clone(), *count);
        });

    println!("{:?}", bigrams);
}

fn get_tags_recovering(port: i32, filename: &str, cat_regex: &Regex) -> Vec<TaggedBigram> {
    match get_tags(port,filename,cat_regex) {
        Ok(result ) => result,
        _  =>  {
            println!("Lemmatizing {} in port {} failed",filename,port);
            get_tags_recovering(port,filename,cat_regex)
        }
    }
}

fn get_tags(port: i32, filename: &str, cat_regex: &Regex) -> reqwest::Result<Vec<TaggedBigram>> {
    let client  = reqwest::Client::builder()
        .timeout(Duration::from_secs(70))
        .build()?;
    let body = read_to_string(filename).unwrap();

    println!("Getting tags for {:?}", filename);
    let mut res = client
        .post(&format!("http://localhost:{}", port))
        .body(body)
        .send()?;

    let body = res.text().unwrap();
    Ok(parse(body, cat_regex))
}

fn parse(input: String, cat_regex: &Regex) -> Vec<TaggedBigram> {
    let with_empty: Vec<&str> = input.split("\n").collect();
    let lines = with_empty
        .iter()
        .map(|x| *x)
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    if lines.len() % 2 != 0 {
        println!("{:?}", lines);

        panic!("Tag response lines should be even");
    } else {
        let mut bigrams: Vec<TaggedBigram> = vec![];
        let mut prev_unigram: Option<Unigram> = Option::None;
        for i in 0..lines.len() / 2 {
            let l1 = lines.get(2 * i).unwrap();
            let opt_cat = parse_cat(l1, lines.get(2 * i + 1).unwrap(), cat_regex)
                .filter(|cat| cat.w.chars().all(|c| c.is_alphabetic()));
            if let Some(cat) = opt_cat {
                if let Some(unigram) = prev_unigram {
                    let bigram = TaggedBigram {
                        w1: unigram,
                        w2: cat.clone(),
                    };
                    bigrams.push(bigram);
                }
                prev_unigram = Some(cat);
            } else {
                prev_unigram = None
            }
        }
        bigrams
    }
}

fn parse_cat(label: &str, s: &str, cat_regex: &Regex) -> Option<Unigram> {
//    let re = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    match cat_regex.captures(s) {
        Option::Some(caps) => Some(Unigram {
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
    //todo: make it compile time?
    let cat_regex = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    assert_eq!(
        Option::Some(Unigram {
            w: "lubić".to_string(),
            cat: "fin".to_string(),
        }),
        parse_cat("", "	lubić	fin:sg:ter:imperf	disamb", &cat_regex)
    );
    assert_eq!(Option::None, parse_cat("", "	interp	disamb'", &cat_regex))
}
