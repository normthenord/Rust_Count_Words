use std::env;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;
use std::{collections::HashMap, fs, io::Write};

const TXT_DIR: &str = "txt_dir";
const HASH_DIR: &str = "hash_dir";

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
        let handle = thread::spawn(|| {
            // println!("Starting thread {text}");
            start_parsing(text);});
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // sleep(Duration::from_secs(10));
}

fn start_parsing(text: String) {
    
    let now = Instant::now();
    // println!("{text}");
    match fs::read_to_string(format!("{}/{}", TXT_DIR, &text)) {
        Ok(file_text) => {
            let name = text.replace(".txt", "_hash.txt");
            let text_map = create_text_map(file_text);
            sort_text_map(text_map, name);
            let elapsed_time = now.elapsed();
            println!(
                "{} took {}ms to parse, count, and sort",
                text,
                elapsed_time.as_millis()
            );
        }
        Err(_) => {
            println!("{text} doesn't exist")
        }
    }
}

fn grab_txts_from_dir(dir: &str) -> Vec<String> {
    let mut txt_vec: Vec<String> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        txt_vec.push(path.unwrap().file_name().into_string().unwrap());
    }
    txt_vec
}

fn create_text_map(file_text: String) -> HashMap<String, u32> {
    let mut text_map: HashMap<String, u32> = HashMap::new();

    for word in file_text.split_whitespace() {
        let count = text_map
            .entry(
                word.replace(
                    &[
                        '(', ')', ',', '\"', '.', ';', ':', '\'', '!', '~', '"', '“', '‘', '`',
                    ],
                    "",
                )
                .to_ascii_lowercase(),
            )
            .or_insert(0);
        *count += 1;
    }
    text_map
}

fn sort_text_map(text_map: HashMap<String, u32>, name: String) {
    let mut count_vec: Vec<(&String, &u32)> = text_map.iter().collect();

    count_vec.sort_unstable_by(|a, b| b.1.cmp(a.1));
    write_hashmap(count_vec, name);
}

fn write_hashmap(count_vec: Vec<(&String, &u32)>, name: String) {
    fs::create_dir_all(HASH_DIR).unwrap();
    let mut new_file = fs::File::create(format!("{}/{}", HASH_DIR, name)).unwrap();

    let mut s = String::new();
    for word in count_vec {
        s = s + &format!("{}: {}\n", word.0, word.1);
    }

    new_file.write_all(s.as_bytes()).unwrap();
}
