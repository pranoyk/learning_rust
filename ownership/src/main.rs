fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);
    s.clear(); // error!
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

