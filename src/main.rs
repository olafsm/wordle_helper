use std::collections::HashMap;
use std::io::stdin;
use std::ops::RangeBounds;
fn main() {
    let input:Vec<&str> = include_str!("../valid_solutions.txt")
        .lines()
        .collect();
        eprintln!("input.len() = {:?}", &input.len());
        let mut most_common = [' ';5];
        let mut most_common2 = [' ';5];
        let mut most_common3 = [' ';5];
        for i in 0..5 {
        let mut alphabet:HashMap<char,i32> = HashMap::<char, i32>::new();
        for word in &input {
            let c = word.chars().nth(i).unwrap();
            if !alphabet.contains_key(&c) {
                alphabet.insert(c, 1);
            } else {
                *alphabet.get_mut(&c).unwrap() += 1;
            }

        }
        let q = alphabet.iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        most_common[i] = *q;
        let mut x = alphabet.clone();
        x.remove(q);
        let q = x.iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        most_common2[i] = *q;

        let mut y = x.clone();
        y.remove(q);
        let q = x.iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        most_common3[i] = *q;

    }
    for word in input {
        //start with sauce
        if check_pos(word, 's', 0) {continue;}
        if !check_pos(word, 'o', 1) {continue;}
        if !check_pos(word, 'i', 2) {continue;}
        if !check_pos(word, 's', 3) {continue;}
        if check_pos(word, ' ', 4) {continue;}
        
        if !word.contains("s") {continue;}
        if word.contains("a") {continue;}
        if word.contains("e") {continue;}
        if word.contains("u") {continue;}
        if word.contains("c") {continue;}
        if word.contains("n") {continue;}
        if word.contains("y") {continue;}

//  k, m
        eprintln!("{:?}", word);
    }
    eprintln!("most_common2 = {:?}", most_common);
    eprintln!("most_common2 = {:?}", most_common2);
    eprintln!("most_common2 = {:?}", most_common3);
}
fn check_pos(w:&str, c:char, n:usize) -> bool {
    return w.chars().nth(n).unwrap() == c;
}