use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    println!("{}", day2(reader))
}

fn day1(mut reader: BufReader<File>) -> i32 {
    //day 1
    let mut count = 0;
    let mut prevNum= reader.by_ref().lines().nth(0).unwrap().unwrap().parse::<i32>().unwrap();
    for line in reader.lines().enumerate(){
        let num: i32 = line.1.unwrap().parse().unwrap();
        if num > prevNum {
            count += 1;
        }
        prevNum = num;

    }
    count
}


fn day2(mut reader: BufReader<File>) -> i32 {
    let mut depth = 0;
    let mut horz = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let pair = line.split(" ").collect::<Vec<&str>>();

        let num = pair[1].parse::<i32>().unwrap();
        if pair[0] == "forward" {
            horz += num
        }
        else {
            depth = if pair[0] == "up" { depth - num } else {depth + num };
        }
    }
    depth * horz
}
