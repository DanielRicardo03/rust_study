enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {

    fn call(&self) {}

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub(crate) fn main() {
    let m = Message::Write(String::from("hello!"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let coin = Coin::Nickel;
    println!("The coin value: {}", value_in_cents(&coin));

    println!("Value: {:?}", plus_one(&None));
    println!("Value: {:?}", plus_one(&Some(10)));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    println!("{:?}", config_max);
}