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

fn task_1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let result = list1
        .iter()
        .zip(list2.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0, |acc, v| v + acc);
    result
}

fn task_2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    // This is a linear approach to solve the problem
    let mut distance = 0;
    let mut start_index = 0;
    let mut count = 0;

    for l in list1 {
        if *l != list2[start_index] {
            if count == 0 {
                start_index += 1;
            } else {
                start_index += count;
                count = 0;
            }
        };
        if count == 0 {
            for r in &list2[start_index..] {
                if r == l {
                    count += 1
                } else {
                    break;
                };
            }
        }
        distance += (count as i32) * *l;
    }
    distance
}

fn task_2_it(list_l: &[i32], list_r: &[i32]) -> i32 {
    list_l
        .iter()
        .map(|elem| {
            let count = list_r.iter().filter(|e| *e == elem).count() as i32;
            let sum = elem * count;
            sum
        })
        .sum()
}

fn main() {
    let (test_input_1, test_input_2) = read_file("input.input");
    // let mut test_input_1: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
    // let mut test_input_2: Vec<i32> = vec![4, 3, 5, 3, 9, 3];
    // test_input_1.sort();
    // test_input_2.sort();

    println!("Input 1: {:#?}", task_1(&test_input_1, &test_input_2));
    println!("Task 2: {:#?}", task_2(&test_input_1, &test_input_2));
    println!("Task 2_it: {:#?}", task_2_it(&test_input_1, &test_input_2));
}
