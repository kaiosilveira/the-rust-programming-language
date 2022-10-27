// Convert strings to pig latim. The first consonant of each word is moved to the end of
// the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a
// vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

enum FirstLetterAnalysisResult {
    Vowel(String),
    Consonant(String),
}

fn main() {
    let example1 = String::from("first");
    let example2 = String::from("car");
    let example3 = String::from("apple");

    let result1 = to_pig_latim(&example1[..]);
    let result2 = to_pig_latim(&example2[..]);
    let result3 = to_pig_latim(&example3[..]);

    println!("{} becomes {}", example1, result1);
    println!("{} becomes {}", example2, result2);
    println!("{} becomes {}", example3, result3);
}

fn to_pig_latim(word: &str) -> String {
    match check_whether_first_letter_is_vowel(&word[..1]) {
        FirstLetterAnalysisResult::Vowel(_) => format!("{}-{}ay", word, "h"),
        FirstLetterAnalysisResult::Consonant(v) => format!("{}-{}ay", &word[1..], &v),
    }
}

fn check_whether_first_letter_is_vowel(letter: &str) -> FirstLetterAnalysisResult {
    if VOWELS.contains(&letter) {
        FirstLetterAnalysisResult::Vowel(String::from(letter))
    } else {
        FirstLetterAnalysisResult::Consonant(String::from(letter))
    }
}
