struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }
}

pub(crate) fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("123@163.com"),
        sign_in_count: 0,
    };

    let user2 = User {
        email: String::from("567@gmail.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;

    let scale = 2;
    let rect = &Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", rect.area());
    println!("rect is: {:#?}", rect);
    dbg!(rect);

    let rect2 = &Rectangle {
        width: 10,
        height: 20,
    };
    println!("Can rect hold rect2?: {}", rect.can_hold(rect2));
    let rect3 = &Rectangle::square(90);
    println!("Can rect hold rect3?: {}", rect.can_hold(rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
