use std::collections::HashMap;
use crate::tag::TaggedBigram;

pub fn hash_to_vec(h:HashMap<TaggedBigram,i32>)-> Vec<(TaggedBigram,i32)>{
    h.into_iter()
        .collect::<Vec<(TaggedBigram,i32)>>()
}

pub fn vec_to_hash(v:Vec<(TaggedBigram,i32)>) -> HashMap<TaggedBigram,i32>{
    let mut hmap = HashMap::new();
    for (k,value) in v{
        hmap.insert(k,value);
    }
    hmap
}