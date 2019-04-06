use std::env;

mod preprocess;
pub mod load_bigrams;
mod pmi;
mod llr;

fn main() {
//    let args: Vec<_> = env::args().collect();
    let args = vec!["pmi".to_string()];
    match args.first().map(|x| x.as_str()) {
        Some("preprocess") => preprocess::preprocess_all(),
        Some("pmi") => pmi::pmis(),
        Some("llr") => llr::llr(),
        _ => panic!("what to do?"),
    }
}
