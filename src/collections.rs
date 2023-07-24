use std::collections::HashMap;
use std::io;
use std::io::stdin;

pub fn main() {
    vectors();
    strings();
    maps();
    median();
    mode();
    pig_latin("first");
    pig_latin("apple");
}

fn vectors() {
    let mut v = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(1);
    v.push(2);

    let third = &v2[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    let first = &v[0];
    println!("The first element is: {first}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 1;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for e in &row {
        println!("{:?}", e);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn strings() {
    let mut s = String::new();
    s.push_str("hello!");

    let s1 = "hello, ".to_string();
    let s2 = "world!".to_string();
    let s1 = s1 + &s2;
    println!("{s1}");

    let s = format!("{s1} - {s2}");
}

fn maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    scores.entry("Red".to_string()).or_insert(0);
    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for world in text.split_whitespace() {
        let wc = map.entry(world).or_insert(0);
        *wc += 1;
    }
    println!("{:?}", map);
}

fn median() {
    let mut v = vec![24, 1, 5, 6, 2, 3, 4, 22, 3];
    v.sort();
    let len = v.len();
    let result = if len % 2 == 0 {
        (v[len / 2] + v[len / 2 - 1]) / 2
    } else {
        v[len / 2]
    };
    println!("{result}");
}

fn mode() {
    let v = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 5, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10];
    let mut m = HashMap::new();
    for num in v {
        let c = m.entry(num).or_insert(0);
        *c += 1;
    }

    let mut num = 0;
    let mut count = 0;
    for (k, v) in m {
        if v > count {
            num = k;
            count = v;
        }
    }
    println!("mode is: {num}");
}

fn pig_latin(s: &str) {
    let first = &s.chars().nth(0).unwrap();
    if ['a', 'e', 'i', 'o', 'u'].contains(first) {
        println!("{s}-hay");
    } else {
        let mut r = String::new();
        for c in s[1..].chars() {
            r.push(c);
        }
        println!("{}-{}ay", r, first);
    }
}
