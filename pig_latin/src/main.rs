use regex::Regex;

fn main() {
    let text = String::from("this is a random text to obtain some results");
    println!("Original text: {}", text);

    let mut words: Vec<String> = Vec::new();

    let begins_with_vowel_regexp = Regex::new(r"(?i)^[aeiou]").unwrap();

    for word in text.split_whitespace().into_iter() {
        let new_word: String;
        match begins_with_vowel_regexp.is_match(word) {
            true => {
                new_word = String::from(word) + "-hay";
            },
            false => {
                new_word = String::from(&word[1..]) + &word[..1] + "-ay";
            }
        }
        words.push(new_word);
    }

    let result = words.join(" ");
    println!("Result text: {}", result);
}
