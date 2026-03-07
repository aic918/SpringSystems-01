mod word_frequency;

use word_frequency::most_frequent_word;

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";

    let (word, count) = most_frequent_word(text);

    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
