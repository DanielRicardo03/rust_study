pub fn main() {
    let number_list = vec![34,50,25,100,65];
    println!("The largest number is: {}", largest(&number_list));
}

fn largest<T>(l: &[T]) -> &T {
    let mut largest = &l[0];
    for num in l {
        if num > largest {
            largest = num;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}