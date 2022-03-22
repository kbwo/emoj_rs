use std::{fs::File, io::Read};

use serde_json::Value;

fn main() {
    println!("Hello, world!");
}

pub fn read_json() -> Result<Value, Box<(dyn std::error::Error + 'static)>> {
    let mut emoji_file = File::open("node_modules/emojilib/dist/emoji-en-US.json")?;
    let mut emoji_file_str = String::new();
    emoji_file.read_to_string(&mut emoji_file_str)?;
    let v: Value = serde_json::from_str(emoji_file_str.as_str())?;
    Ok(v)
}

pub fn search<'a>(query: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    v.push(query);
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_emoji() {
        let query = "face";
        assert_eq!([query].to_vec(), search(query))
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
