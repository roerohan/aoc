use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let filename = "input.txt";
    let lines = read_lines(&filename);

    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    for line in lines {
        let a = line
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();

        let num1 = a[0].parse().unwrap();
        let num2 = a[1].parse().unwrap();
        arr1.push(num1);
        arr2.push(num2);
    }

    arr1.sort();
    arr2.sort();

    let mut sum = 0;

    for i in 0..arr1.len() {
        sum += (arr1[i] - arr2[i]).abs();
    }

    println!("{:?}", sum);
}
