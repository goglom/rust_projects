use std::io;

fn is_vowel(symbol: &char) -> bool {
    match symbol.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went fron while reading stdin");

    let input = input.trim();

    let mut result = String::new();

    let mut iter = input.chars();
    if let Some(fst) = iter.next() {
        if !fst.is_alphabetic() {
            println!("Expected alphabetic string but hot something else:(");
            return;
        }

        if is_vowel(&fst) {
            result.push_str(&input);
            result.push_str("-hay");
        } else {
            result.push_str(iter.as_str());
            result.push_str(&format!("-{}ay", fst));
        }
    }
    println!("Yout pig-latin word: \"{}\"", result);
}
