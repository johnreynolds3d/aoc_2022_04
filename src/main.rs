use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut min1: u16;
    let mut max1: u16;
    let mut min2: u16;
    let mut max2: u16;

    let mut score: u16 = 0;

    for line in lines {
        let v: Vec<_> = line.as_ref().unwrap().split([',', '-']).collect();

        min1 = v[0].parse().unwrap();
        max1 = v[1].parse().unwrap();
        min2 = v[2].parse().unwrap();
        max2 = v[3].parse().unwrap();

        if (max1 - min1) + 1 >= (max2 - min2) + 1 {
            if min2 >= min1 && max2 <= max1 {
                score += 1;
            }
        } else {
            if min1 >= min2 && max1 <= max2 {
                score += 1;
            }
        }
    }
    println!("Score: {}", score);

    Ok(())
}
