use std::env;
use std::process::exit;

use clap::{App, Arg, SubCommand};

mod preprocess;
mod llr;
mod tag;
pub mod utils;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now();

    // ala();
    // let end = SystemTime::now();
    // let millis = end.duration_since(UNIX_EPOCH).unwrap() - start.duration_since(UNIX_EPOCH).unwrap();
    // println!("Worked: {:?}", millis);

    // exit(1);

    //Fixme - not best Cli experience
    let matches = App::new("Lab5")
        .arg(Arg::with_name("preprocess")
            .long("preprocess")
            .help("Sanitizes input"))
        .arg(Arg::with_name("llr")
            .short("p")
            .long("llr")
            .help("Computes llr"))
        .arg(Arg::with_name("tags")
            .short("t")
            .long("tag")
            .help("Tags corpus"))
        .get_matches();


    if matches.occurrences_of("llr") > 0 {
        println!("llr");
       llr::llr();
    } else if matches.occurrences_of("tags") > 0 {
        println!("tags");
        println!("Not implemented");
    } else if matches.occurrences_of("preprocess") > 0 {
        preprocess::preprocess_all();
    }


    let end = SystemTime::now();
    let millis = end.duration_since(UNIX_EPOCH).unwrap() - start.duration_since(UNIX_EPOCH).unwrap();
    println!("Worked: {:?}", millis);
}

fn ala() {
    tag::test_paralell();
}


