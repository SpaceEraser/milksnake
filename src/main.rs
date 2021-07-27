use rayon::prelude::*;

fn words() -> std::result::Result<Vec<String>, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    BufReader::new(File::open("wordlist.txt")?)
        .lines()
        .filter(|rl| rl.as_ref().map(|l| l.len() >= 4).ok().unwrap_or(false))
        .collect()
}

fn word_makes_sense(word: &str, wordlist: &[String]) -> i32 {
    for w in wordlist {
        if !word.starts_with(w) {
            continue;
        }
        if word.len() == w.len() {
            return 0;
        }
        let rec_call = word_makes_sense(&word[w.len()..], wordlist);
        if rec_call >= 0 {
            return rec_call + 1;
        }
    }
    return -1;
}

fn similar_letters() -> std::collections::HashMap<&'static str, Vec<&'static str>> {
    static CHAR_LIST: &[[&'static str; 2]] = &[
        ["b", "d"],
        ["c", "o"],
        ["h", "n"],
        ["n", "m"],
        ["p", "q"],
        ["s", "z"],
        ["w", "m"],
        ["i", "j"],
        ["i", "l"],
        ["u", "v"],
        ["o", "q"],
        ["g", "q"],
        ["rn", "m"],
        ["ln", "h"],
    ];
    let mut ret = std::collections::HashMap::new();
    for &[a, b] in CHAR_LIST {
        ret.entry(a).or_default().push(b);
        ret.entry(b).or_default().push(a);
    }
    ret
}

fn main() {
    let words = words().unwrap();
    let similar_letters = similar_letters();
    let similar_words = words.par_iter().flat_map(|cur_word| {
        let mut word = cur_word.chars().collect::<Vec<_>>();
        let mut local_similar_words = Vec::new();
        for i in 0..word.len() {
            if let Some(matching_chars) = similar_letters.get(&word[i]) {
                for &m in matching_chars {
                    let b = word[i];
                    word[i] = m;
                    let word_str = word.iter().collect::<String>();
                    if word_makes_sense(&*word_str, &*words) > 0 {
                        local_similar_words.push((cur_word.clone(), word_str));
                    }
                    word[i] = b;
                }
            }
        }
        return local_similar_words;
    }).collect::<Vec<_>>();
    dbg!(&similar_words);
    dbg!(similar_words.len());
}
