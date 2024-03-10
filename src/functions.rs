
use std::env::args;

fn receiver() -> Vec<String> {

    let args: Vec<String> = args().collect();

    args[1..].to_vec()

}

pub fn echo_rs() -> String {

    let text: Vec<String> = receiver();
    let mut string: String = String::new();

    for i in text.iter() {
        
        string = format!("{string} {i}");

    }

    print!("{}", string);
    string

}

use std::path::Path;
use std::fs;

pub fn cat_rs() {

    let file_path: &String = &receiver()[0];

    let text: String = fs::read_to_string(file_path).expect("Error: it wasn't possible to read the file.");
    
    print!("{}", text);

}

pub fn ls_rs() {

    let path_str: &String = &receiver()[0];
    let path = Path::new(&path_str);

    for i in path.read_dir() {

        dbg!(i);
    }
}
