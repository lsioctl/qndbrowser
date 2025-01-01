use qndengine::token;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let token_list = token::tokenizer(&content);

    println!("{:?}", token_list);
}
