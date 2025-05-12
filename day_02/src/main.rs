use std::fs::{self, File};
use std::io::Read;
use std::result;

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(path).expect("IO Error!");
    let mut result: Vec<_> = content
        .split('\n')
        .map(|line|
            line.split(' ')
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect();
    result
}

fn task_1(input: &Vec<Vec<i32>>) -> usize {
    input
        .into_iter()
        .map(|vec| {
            let mut iter = vec.into_iter().peekable();
            let mut monoton = 0;
            let mut max_distance = 0;
            while let Some(value) = iter.next() {
                if let Some(num) = iter.peek() {
                    monoton |= if *value < **num { 1 } else { 0 };
                    monoton |= if *value > **num { 2 } else { 0 };
                    monoton |= if *value == **num { 4 } else { 0 };
                    let distance = (*value - **num).abs();
                    if distance > max_distance {
                        max_distance = distance
                    }
                }
            }
            // Two criteria: Monotony and distance is less then 4
            (monoton == 2 || monoton == 1, max_distance < 4)
        })
        .filter(|(x, y)| *x && *y)
        .count()
}

fn main() {
    let example_data = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];

    println!("Number of safe of example: {}", task_1(&example_data));
    println!("Number of safe of example: {}", task_1(&read_file("input.input")));

}
