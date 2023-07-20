mod functions;
mod ownership;
mod structs;
mod enums;

use std::io;
use crate::ownership::ownership_learn;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of is: {x}");

    // let a = [1,2,3,4,5];
    // println!("Please enter an array index.");
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("Failed to read line");
    // let index: usize = index.trim().parse().expect("Index entered was not a number");
    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");

    functions::another_function(6, 'h');

    let y = {
        let x = 1+1;
        x + 2
    };
    println!("Y: {y}");

    // call fun
    let five = functions::five();   // trailing comments
    println!("Five: {five}");

    // control flow
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loops
    let mut counter = 3;
    while counter != 0 {
        println!("{counter}");
        counter -= 1;
    }
    println!("lift off!");

    // Fibonacci
    // let mut first = 0;
    // let mut second = 1;
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("input `n` of Fibonacci");
    // let n: usize = index.trim().parse().expect("Index entered was not a number");
    // for _ in 0..n+1 {
    //     print!("{first} ");
    //     let result = first + second;
    //     first = second;
    //     second = result;
    // };

    println!("-------------- ownership ----------------");
    ownership_learn();

    println!("-------------- structs __________________");
    structs::main();
}
