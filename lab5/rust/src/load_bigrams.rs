use std::fs::File;
use std::io::Read;

use serde;
use serde_pickle;
use serde_pickle as pickle;
use serde_pickle::{HashableValue, Value};

#[derive(Debug,Clone)]
pub struct Entry {
    pub w1: String,
    pub w2: String,
    pub count: i64,
}

fn w1w2_to_str(w1w2: &Vec<HashableValue>) -> Vec<&str> {
    w1w2.iter().map(|x| match x {
        HashableValue::String(string) => string.as_str(),
        _ => panic!("Oh god!")
    }).collect::<Vec<&str>>()
}

pub fn load_entries() -> Vec<Entry>{
    let reader: Box<Read> = Box::new(File::open("../bigrams_dict.out").unwrap());

    let decoded: pickle::Value = pickle::value_from_reader(reader).unwrap();

    let dict = match decoded {
        pickle::Value::Dict(btreemap) => btreemap,
        _ => panic!("Damnn")
    };

    dict.iter().map(|entry| match entry {
         (HashableValue::Tuple(w1w12), Value::I64(count)) =>
             {
                 let v = w1w2_to_str(w1w12);
                 Entry { w1: v.first().unwrap().to_string(), w2: v.get(1).unwrap().to_string(), count: *count }
             }
         _ => panic!("Invalid tuple")
     }).collect::<Vec<Entry>>()
}
