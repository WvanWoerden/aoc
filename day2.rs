use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;


fn scoreplay1(ip:String) -> i32 {
	let s1 = ip.chars().nth(0).unwrap();
	let s2 = ip.chars().nth(2).unwrap();
	let v1 = (s1 as i32) - 65i32;
	let v2 = (s2 as i32) - 88i32;
	
	let score = v2 + 1;
	match v2-v1 {
		0 => score+3,
		1 | -2 => score+6,
		_ => score,
	}
}

fn scoreplay2(ip:String) -> i32 {
	let s1 = ip.chars().nth(0).unwrap();
	let s2 = ip.chars().nth(2).unwrap();
	let v1 = (s1 as i32) - 65i32;
	let v2 = (s2 as i32) - 88i32;
	3*v2 + (v1+v2+2)%3 + 1
}

fn part1(){
	println!("Hello world!");

	let mut score = 0;

	if let Ok(lines) = read_lines("input2.txt") {
		for line in lines {
			if let Ok(ip) = line {
				score += scoreplay2(ip);
				println!("{score}\n");
			}
		}
	}
	println!("score: {score}\n");
}

fn main() {
	part1();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}