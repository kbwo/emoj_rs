use regex::Regex;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

fn read_json() -> Result<Value, Box<(dyn std::error::Error)>> {
    let mut emoji_file = File::open("node_modules/emojilib/dist/emoji-en-US.json")?;
    let mut emoji_file_str = String::new();
    emoji_file.read_to_string(&mut emoji_file_str)?;
    let v: Value = serde_json::from_str(emoji_file_str.as_str())?;
    Ok(v)
}

pub fn search(query: &str) -> Result<Vec<String>, Box<(dyn std::error::Error)>> {
    let reg_empty = Regex::new(r"\s").unwrap();
    let reg_not_word = Regex::new(r"\W").unwrap();
    let lowercase = query.to_lowercase();
    let regex_source = reg_empty
        .split(&lowercase)
        .map(|x| {
            reg_not_word.replace(x, "");
            x
        })
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>()
        .join("|");
    let main_regex = Regex::new(regex_source.as_str()).unwrap();
    let mut matched_emojis: Vec<String> = Vec::new();
    let json_value = read_json()?;
    let json_object = json_value.as_object().unwrap();
    for (k, v) in json_object {
        let values = v
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| -> &str { x.as_str().unwrap() })
            .filter(|text| -> bool { main_regex.is_match(text) })
            .collect::<Vec<&str>>();
        if !values.is_empty() {
            matched_emojis.push(k.as_str().to_string());
        }
    }
    Ok(matched_emojis)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_emoji() {
        let query = "face";
        match search(query) {
            Ok(result) => {
                let is_contain = result.contains(&"😀".to_string());
                assert_eq!(true, is_contain);
            }
            Err(e) => {
                eprintln!("Something went wrong on searching: {}", e);
            }
        }
    }

    #[test]
    fn can_read_json() {
        match read_json() {
            Ok(v) => {
                let json_object = v.as_object().unwrap();
                for (k, v) in json_object {
                    println!("key={}, Value={}", k, v);
                    let first_emoji_desc = [
                        "grinning_face",
                        "face",
                        "smile",
                        "happy",
                        "joy",
                        ":D",
                        "grin",
                    ];
                    if k == "😀" {
                        assert_eq!(v.as_array().unwrap().to_vec(), first_emoji_desc);
                    } else {
                        assert_ne!(v.as_array().unwrap().to_vec(), first_emoji_desc);
                    }
                }
            }
            Err(e) => {
                eprintln!("Something went wrong: {}", e);
            }
        }
    }
}
