use std::time::Instant;
use std::{collections::HashMap, fs, io::Write};

pub const TXT_DIR: &str = "txt_dir";
pub const HASH_DIR: &str = "hash_dir";

#[derive(Debug)]
pub struct ParseResult {
    pub text: String,
    pub elapsed_time: u128,
    pub unique_words: usize,
    pub total_words: u32,
}


pub fn start_parsing(text: String) -> Result<ParseResult, String> {
    let now = Instant::now();
    match fs::read_to_string(format!("{}/{}", TXT_DIR, &text)) {
        Ok(file_text) => {
            let name = text.replace(".txt", "_hash.txt");
            let text_map = create_text_map(file_text);
            sort_text_map(&text_map, name);
            let result = ParseResult {
                text,
                elapsed_time: now.elapsed().as_millis(),
                unique_words: text_map.iter().count(),
                total_words: text_map.iter().map(|(_x, y)| y).sum::<u32>(),
            };
            return Ok(result);
        }
        Err(_) => {
            println!("{text} doesn't exist");
            Err("Text doesn't exist".to_owned())
        }
    }
}

pub fn grab_txts_from_dir(dir: &str) -> Vec<String> {
    let mut txt_vec: Vec<String> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        txt_vec.push(path.unwrap().file_name().into_string().unwrap());
    }
    txt_vec
}

pub fn create_text_map(file_text: String) -> HashMap<String, u32> {
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

pub fn sort_text_map(text_map: &HashMap<String, u32>, name: String) {
    let mut count_vec: Vec<(&String, &u32)> = text_map.iter().collect();

    count_vec.sort_unstable_by(|a, b| b.1.cmp(a.1));
    write_hashmap(count_vec, name);
}

pub fn write_hashmap(count_vec: Vec<(&String, &u32)>, name: String) {
    fs::create_dir_all(HASH_DIR).unwrap();
    let mut new_file = fs::File::create(format!("{}/{}", HASH_DIR, name)).unwrap();

    let mut s = String::new();
    for word in count_vec {
        s = s + &format!("{}: {}\n", word.0, word.1);
    }
    new_file.write_all(s.as_bytes()).unwrap();
}
