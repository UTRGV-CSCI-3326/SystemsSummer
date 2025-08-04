fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut unique_words: Vec<&str> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    for &word in &words {
        let mut found = false;
        for (i, &w) in unique_words.iter().enumerate() {
            if w == word {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique_words.push(word);
            counts.push(1);
        }
    }
        let mut max_index = 0;
        let mut max_count = 0;
        for (i, &count) in counts.iter().enumerate() {
            if count > max_count {
                max_count = count;
                max_index = i;
            }
        }

    (unique_words[max_index].to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}