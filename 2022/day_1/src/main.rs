use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./resources/input.txt") {
        let mut sum = 0;
        let mut sums = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip.eq("") {
                    sums.push(sum);
                    sum = 0;
                    continue;
                }
                sum += ip.parse::<i32>().unwrap();
            }
        }
        sums.sort();
        let i = sums.len()-1;
        println!("Amount of calories carried by the top elf: {}", sums[i]);
        println!("Amount of calories carried by the top three elf: {}", sums[i]+sums[i-1]+sums[i-2]);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
