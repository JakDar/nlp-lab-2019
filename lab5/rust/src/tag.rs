use reqwest;
use regex::Regex;
use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TaggedBigram {
    pub w1: String,
    pub w1_cat: String,
    pub w2: String,
    pub w2_cat: String,
}

#[derive(Debug,PartialEq,Eq)]  // could be optmized by using enums as categories
pub struct Unigram {
    pub w: String,
    pub cat: String,
}




pub fn test() {
    let client = reqwest::Client::new();

    let body =  read_to_string("../../lower_ustawy/1994_288.txt").unwrap();

    let mut res = client.post("http://localhost:9200")
        .body(body)
        .send().unwrap();

    let body = res.text().unwrap();
    let ala = parse(body.clone());
//    println!("{}", body);
}


fn parse(input: String) -> Vec<TaggedBigram> {
    let with_empty: Vec<&str> = input.split("\n").collect();
    let lines = with_empty.iter().map(|x| *x).filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    if lines.len() % 2 != 0 {
        println!("{:?}", lines);

        panic!("Tag response lines should be even");
    } else {


        //todo:bcm - fold bigrams
        for i in 0..lines.len() / 2 {
            println!("{}", lines.get(2 * i).unwrap());
            println!("{}", lines.get(2 * i + 1).unwrap());
            let l1: Vec<&str> = lines.get(2 * i).unwrap().split("\t").collect();
//            let w = *l1.first().unwrap();
            let cat = parse_cat(lines.get(2 * i + 1).unwrap());
            println!("{:?}",cat);


            println!("-----------------------");
        }

    }
    panic!("{}")


}


fn parse_cat(s: &str) -> Unigram{
    let re = Regex::new(r"\t(?P<lemma>\w.*?)\t(?P<cat>\w+?)[:\t]").unwrap();
    let caps = re.captures(s).unwrap();
    Unigram{w: caps["lemma"].to_string(),
    cat: caps["cat"].to_string()
    }
}

#[test]
fn parse_cat_test() {
    assert_eq!(Unigram{w:"lubić".to_string(),cat:"fin".to_string()}, parse_cat("	lubić	fin:sg:ter:imperf	disamb"))
}


