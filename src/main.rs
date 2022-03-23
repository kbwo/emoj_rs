use emoj_rs::*;
use std::sync::Arc;

use arboard::Clipboard;
use tuikit::attr::{Attr, Color, Effect};
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::raw::IntoRawMode;
use tuikit::term::{Term, TermHeight};

fn main() {
    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut search_query = String::new();
    let mut selected_index: usize = 0;
    let mut search_result: Vec<String> = vec![];
    let term: Arc<Term<()>> = Arc::new(Term::with_height(TermHeight::Fixed(3)).unwrap());
    while let Ok(ev) = term.poll_event() {
        let _ = &term.clear();
        match ev {
            Event::Key(Key::Enter) => {
                if &search_result.len() - 1 >= selected_index {
                    let target = &search_result[selected_index];
                    let mut clipboard = Clipboard::new().unwrap();
                    clipboard.set_text(target.to_owned()).unwrap();
                    //row
                    for i in 0..3 {
                        //col
                        for j in 0..18 {
                            let _ = term.print(i, j, "");
                        }
                    }
                    let _ = term.present();
                    break;
                }
            }
            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Right) => {
                selected_index = if selected_index >= &search_result.len() - 1 {
                    &search_result.len() - 1
                } else {
                    selected_index + 1
                }
            }
            Event::Key(Key::Left) => {
                selected_index = if selected_index == 0 || search_result.is_empty() {
                    0
                } else {
                    selected_index - 1
                }
            }
            Event::Key(Key::Backspace) | Event::Key(Key::Delete) | Event::Key(Key::Null) => {
                search_query.pop();
            }
            _ => {
                if let Event::Key(Key::Char(value)) = ev {
                    search_query.push_str(value.to_string().as_str());
                }
            }
        }
        reset_index(&search_result, &mut selected_index);
        print_query(&term, search_query.to_string());
        if let Ok(res) = search::search(&search_query.to_string().as_str()) {
            search_result = res;
            search_result = limit_emojis(&search_result).to_vec();
        }
        print_result(&term, &search_result, selected_index);
    }
    if let Err(e) = term.clear_on_exit(false) {
        panic!("some terminal settings went wrong: {}", e);
    };
    println!(
        "{} has been copied to the clipboard",
        search_result[selected_index]
    );
}

fn reset_index(result: &Vec<String>, index: &mut usize) {
    if result.is_empty() {
        *index = 0;
    } else if result.len() - 1 < *index {
        *index = result.len();
    }
}

fn print_query(term: &Term, query: String) {
    let _ = term.clear();
    let _ = term.print(0, 0, query.as_str());
    let _ = term.present();
}

fn print_result(term: &Term, result: &Vec<String>, selected_index: usize) {
    for each_emoji in result {
        let position = result.iter().position(|x| x == each_emoji).unwrap_or(0);
        if each_emoji == &result[selected_index] {
            let attr = Attr {
                fg: Color::GREEN,
                effect: Effect::UNDERLINE,
                ..Attr::default()
            };
            let _ = term.print_with_attr(1, position * 5, each_emoji.as_str(), attr);
        } else {
            let _ = term.print(1, position * 5, each_emoji.as_str());
        }
    }
    let _ = term.present();
}

/// max numbers of emojis are 7
fn limit_emojis(searched_emojis: &Vec<String>) -> &[String] {
    if searched_emojis.len() > 7 {
        &searched_emojis[0..7]
    } else {
        &searched_emojis[..]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn limit_numbers_of_str() {
        let test_strings = ["😀", "😃", "c", "d", "e", "f", "g", "h", "i"]
            .map(|x| x.to_string())
            .to_vec();
        let limited_strings = limit_emojis(&test_strings);
        // assert_eq!(7, limited_strings.len());
        assert_eq!(["😀", "😃", "c", "d", "e", "f", "g"], limited_strings);
    }

    #[test]
    fn numbers_of_tiny_str() {
        let test_strings = ["a", "b", "c", "d"].map(|x| x.to_string()).to_vec();
        let limited_strings = limit_emojis(&test_strings);
        assert_eq!(4, limited_strings.len());
    }
}
