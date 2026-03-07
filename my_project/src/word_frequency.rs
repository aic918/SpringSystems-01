pub fn most_frequent_word(text: &str) -> (String, usize) {

    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count = 0;

    for i in 0..words.len() {
        let mut count = 0;

        for j in 0..words.len() {
            if words[i] == words[j] {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = words[i];
        }
    }

    (max_word.to_string(), max_count)
}