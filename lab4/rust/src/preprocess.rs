use std::ffi::OsString;
use std::fs::{create_dir, File, read_to_string};
use std::io::Error;
use std::io::prelude::*;

//todo - could add error handling and parallelize

pub fn ls(path: &str)  -> Vec<OsString>{
    use std::fs;
    let dir = fs::read_dir(path).unwrap().into_iter();

    dir.map(|x| x.unwrap().path().as_os_str().to_os_string()).collect::<Vec<OsString>>()
}

//fn is_known(c:char) -> bool{
//    c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace() || "§”„–".contains(c)
//}

fn is_punctuation_in_corpus(c:char) -> bool{
    c.is_ascii_punctuation()  || "§”„–".contains(c)
}

fn preprocess(str:String) -> String{

//    str.chars().filter(|x| !is_punctuation_in_corpus(*x)).map(|x| x.to_lowercase().to_string()).collect::<String>()
    str.chars()
        .map(|x| if is_punctuation_in_corpus(x) {" ".to_string()} else {x.to_lowercase().to_string()} )
        .collect::<String>()

}

fn preprocess_file(in_filename:&str, out_filename:&str)-> Result<(),Error>{
    let content = read_to_string(in_filename).unwrap();
    let parsed = preprocess(content);
    let bytes = parsed.as_bytes();
    let mut out_file = File::create(out_filename).unwrap();
    out_file.write_all(bytes)


}

pub fn preprocess_all() {
    let dir =ls("../../ustawy");
    let head = dir.first().unwrap().to_str().unwrap();
    println!("{}",head);

    create_dir("../../lower_ustawy").unwrap();

    for ospath in dir{
        let path= ospath.to_str().unwrap();
        let filename:&str = path.split("/").last().unwrap();
        println!("{}",filename);
        let out_filename ="../../lower_ustawy/".to_owned()+ filename;


        preprocess_file(path,&out_filename).unwrap();
    }


}
