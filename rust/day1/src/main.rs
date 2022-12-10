use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// Fair warning, I am still learning, this is duct taped together
fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut max = 0;
    let mut total = 0;
    let mut top_3: [i32; 3] = [0, 0, 0];

    for line in reader.lines() {
        if let Ok(val) = line {
            match val.as_str() {
                "" => {
                    top_3 = populate_top_3(top_3, total);
                    if total > max {
                        max = total.clone();
                    }
                    total = 0;
                },
                _ => {
                    total += val.parse::<i32>().unwrap();
                },
            }
        }
    }
    let sum: i32 = top_3.iter().sum();
    println!("Max is {}", max);
    println!("Top 3: {}, {}, {} [Sum: {}]", top_3[0], top_3[1], top_3[2], sum);
    Ok(())
}

// Trying to keep constant memory
fn populate_top_3(mut top_3: [i32; 3], input: i32) -> [i32; 3]{
    for (index, val) in top_3.iter().enumerate() {
        if input > *val {
            for i in (index + 1..top_3.len()).rev() {
                top_3[i] = top_3[i - 1];
            }
            top_3[index] = input;
            return top_3;
        }
    }
    top_3
}