fn main() {
    println!("Hello, world!");
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
}
