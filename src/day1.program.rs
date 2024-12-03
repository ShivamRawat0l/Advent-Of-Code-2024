
use std::fs;

pub fn day1() {
    let contents = fs::read_to_string("src/day1.input.txt").unwrap();
    let mut a = Vec::<i32>::new();
    let mut b = Vec::<i32>::new();
    contents.split("\n").for_each(|s| {
        let parts = s.split("   ").collect::<Vec<&str>>();
        a.push(parts[0].parse::<i32>().unwrap());
        b.push(parts[1].parse::<i32>().unwrap());
    });
    a.sort();
    b.sort();
    let mut distance = 0;

    for i in 0..a.len() {
        distance += (a[i] - b[i]).abs();
    }
    println!("Distance is {:?}", distance);
}