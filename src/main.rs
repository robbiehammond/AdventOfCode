use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    //day 1
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut count = 0;
    let mut prevNum= reader.by_ref().lines().nth(0).unwrap().unwrap().parse::<i32>().unwrap();
    for line in reader.lines().enumerate(){
        let num: i32 = line.1.unwrap().parse().unwrap();
        if num > prevNum {
            count += 1;
        }
        prevNum = num;

    }
    println!("{}", count);
}
