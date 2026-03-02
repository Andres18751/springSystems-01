fn most_used_word(text: &str) -> (String, usize){
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut word_counts: Vec<(String, usize)> = Vec::new();
    for &word in &words {
        
        let mut found = false;
        for (stored_word, count) in &mut word_counts {
            if stored_word == word {
                *count += 1;         
                found = true;
                break;
            }
        }
        
        if !found {
            word_counts.push((word.to_string(), 1));
        }
    }
    let mut max_word = String::new();
    let mut max_count = 0;
    for (word, count) in &word_counts {
        if *count > max_count {
            max_count = *count;
            max_word = word.clone();  // clone to own the string
        }
    }
    (max_word,max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_used_word(text);
    println!("Most frequent word \"{}\" ({} times)", word, count);
}
