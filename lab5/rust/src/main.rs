use std::env;
use clap::{Arg, App, SubCommand};
use std::process::exit;

mod preprocess;
pub mod load_bigrams;
mod pmi;
mod llr;
mod tag;


fn main() {

    ala();
    exit(1);

    //Fixme - not best Cli experience
    let matches = App::new("Lab5")
        .arg(Arg::with_name("preprocess")
            .long("preprocess")
            .help("Sanitizes input"))
        .arg(Arg::with_name("pmi")
            .short("p")
            .long("pmi")
            .help("Computes pmi"))
        .arg(Arg::with_name("tags")
            .short("t")
            .long("tag")
            .help("Tags corpus"))
        .get_matches();


    if matches.occurrences_of("pmi") > 0 {
        println!("pmi");
        pmi::pmis();
    } else if matches.occurrences_of("tags") > 0 {
        println!("tags");
        println!("Not implemented");
    } else if matches.occurrences_of("preprocess") > 0 {
        preprocess::preprocess_all();
    }

    // more program logic goes here...
}

fn ala() {
//    let args: Vec<_> = env::args().collect();


    tag::test();


//    let args = vec!["pmi".to_string()];
//    match args.first().map(|x| x.as_str()) {
//        Some("preprocess") => preprocess::preprocess_all(),
//        Some("pmi") => pmi::pmis(),
//        Some("llr") => llr::llr(),
//        _ => panic!("what to do?"),
//    }
}
