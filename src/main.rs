use std::env;
use std::thread;

mod utils;
use utils::*;


fn main() {
    let mut text_vec: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        text_vec.append(&mut grab_txts_from_dir(TXT_DIR));
    } else {
        for arg in &args[1..] {
            text_vec.push(arg.to_string());
        }
    }

    let mut handles = vec![];

    for text in text_vec {
        let handle = thread::spawn(|| start_parsing(text));
        handles.push(handle);
    }
    // let mut result_vec = vec![];
    let mut result_vec  = vec![];
    for handle in handles {
        let result = handle.join().unwrap();
        match result {
            Ok(response) => result_vec.push(response),
            Err(s) => {
                println!("{s}");
            }
        }
    }
    //Sort result_vec by elapsed time
    result_vec.sort_by(|a, b| a.elapsed_time.cmp(&b.elapsed_time));
    
    for result in result_vec {
        println!("{} took {}ms to parse, count, and sort -- It includes {} unique words and {} total words", 
        result.text, 
        result.elapsed_time, 
        result.unique_words, 
        result.total_words)
    }
}

