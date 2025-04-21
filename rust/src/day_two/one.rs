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

    let mut safe = 0;
    for line in lines {
        let arr = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        if arr.len() < 2 {
            safe += 1;
            continue;
        }

        let ascending = arr[1] > arr[0];
        let mut is_safe = false;

        for i in 1..arr.len() {
            if ascending {
                if (arr[i] - arr[i - 1] > 3) || (arr[i] - arr[i - 1] < 1) {
                    is_safe = false;
                    break;
                }
            } else if arr[i - 1] - arr[i] > 3 || arr[i - 1] - arr[i] < 1 {
                is_safe = false;
                break;
            }
            is_safe = true;
        }

        if is_safe {
            println!("Safe: {:?}", arr);
            safe += 1;
        }
    }

    println!("Num safe: {:?}", safe);
}
