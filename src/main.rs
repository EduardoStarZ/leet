use std::fs::File;
use std::io::Write;
use std::fs;

mod funnel;

fn main() {
    let filepaths : Vec<String> = fetch_files();

    filepaths.iter().for_each(|path| println!("{path}"));
}

fn split_names(pathname: String) -> String {
    pathname.replace("./build/", "")
}

fn fetch_files() -> Vec<String> {
    let paths = fs::read_dir("./build/").unwrap();
    let mut files : Vec<String> = Vec::new(); 
    
    for x in paths {
        // println!("{}", x.unwrap().path().display());
        if x.as_ref().unwrap().path().display().to_string().contains(".rs") {
            files.push(x.unwrap().path().display().to_string());
        }
    }

    files
}

fn modify() {
    let mut funnel: File = File::create("funnel.rs").expect("couldn't create the file");

    funnel.write("".as_bytes()).expect("couldn't write into the file");
}