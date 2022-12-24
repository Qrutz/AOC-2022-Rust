use std::env;
use std::fs;

fn getFile() -> String {
    let contents = fs::read_to_string("src/1.txt").expect("Should have been able to read the file");

    return contents;
}

// Part 1
fn getRichestElf(s: &str) -> i32 {
    let mut sum = 0;
    let mut max = 0;
    for line in s.lines() {
        if !line.is_empty() {
            let z = line.parse::<i32>().unwrap();
            sum += zi;
        }

        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        }
    }
    return max;
}

fn part2() {
    let contents = getFile();
    let mut sum = 0;
    let mut max = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for line in contents.lines() {
        if !line.is_empty() {
            let z = line.parse::<i32>().unwrap();
            sum += z;
        }

        if line.is_empty() {
            if sum > max {
                max3 = max2;
                max2 = max;
                max = sum;
            } else if sum > max2 {
                max3 = max2;
                max2 = sum;
            } else if sum > max3 {
                max3 = sum;
            }
            sum = 0;
        }
    }
    let sum = max + max2 + max3;
    println!("Sum of 3 biggest elfs: {}", sum);
}

//

fn main() {
    let contents = getFile();
    let max = getRichestElf(&contents);
    part2();
}
