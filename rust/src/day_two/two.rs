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

        let mut diffs = Vec::new();
        let ascending = arr[0] < arr[1];

        for i in 1..arr.len() {
            diffs.push(arr[i] - arr[i - 1])
        }

        // println!("Diffs, {:?}", diffs);
        let diffs_clone = diffs.clone();

        let mut tolerance = 1;
        for i in 0..diffs.len() - 1 {
            if ascending {
                if diffs[i] <= 3 && diffs[i] >= 1 {
                } else {
                    if diffs[i] > 3 && diffs[i + 1] < 3 - diffs[i] {
                        tolerance -= 1;
                    } else {
                        tolerance -= 2;
                        break;
                    }

                    if diffs[i] < 1 && diffs[i + 1] > 1 - diffs[i] {
                        tolerance -= 1;
                    } else {
                        tolerance -= 2;
                        break;
                    }
                }
            } else {
                if diffs[i] >= -3 && diffs[i] <= -1 {
                } else {
                    if diffs[i] < -3 && diffs[i + 1] > -3 - diffs[i] {
                        tolerance -= 1;
                    } else {
                        tolerance -= 2;
                        break;
                    }

                    if diffs[i] > -1 && diffs[i + 1] < -1 - diffs[i] {
                        tolerance -= 1;
                    } else {
                        tolerance -= 2;
                        break;
                    }
                }
            }
        }

        if tolerance >= 0 {
            safe += 1;
            println!("{:?}, {:?}", arr, diffs_clone);
        }
    }

    println!("Num safe: {:?}", safe);
}
