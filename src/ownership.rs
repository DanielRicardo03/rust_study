pub(crate) fn ownership_learn() {
    let s = "hello";
    let mut s = String::from(s);
    s.push_str(", world!");
    println!("{}", s);
    takes_ownership(&s);
    println!("{s}");
    println!("{}", calculate_length(&s));

    let s1 = gives_ownership();
    let s2 = s1;
    let _s3 = takes_and_gives_back(s2);

    let sl1 = &s[..2];
    println!("{sl1}");

    let a = [1,2,3,4,5];
    assert_eq!(&a[1..3], &[2,3])
}

fn takes_ownership(str: &str) {
    println!("{str}");
}

fn gives_ownership() -> String {
    let some_str = String::from("yours");
    some_str
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("123");
}