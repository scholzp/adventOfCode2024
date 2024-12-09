use std::fs::{self, File};
use std::io::Read;
use std::result;

fn read_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let content = fs::read_to_string(path).expect("IO Error!");
    let mut tuple: (Vec<i32>, Vec<i32>) = content
        .split('\n')
        .map(|v| v.split_once("   ").expect("Couldn't split"))
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .unzip();
    tuple.0.sort();
    tuple.1.sort();
    tuple
}

fn main() {
    let (test_input_1, test_input_2) = read_file("input.input");
    // let mut test_input_1: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
    // let mut test_input_2: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
    // test_input_1.sort();
    // test_input_2.sort();
    let result = test_input_1
        .iter()
        .zip(test_input_2.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0, |acc, v| v + acc);
    println!("Input 1: {:#?}", result);
}
