use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::str::FromStr;

fn processline(ip:&String) -> i32 {
	let mut p : Vec<i32> = ip.split(&['-', ','][..]).map(|x| i32::from_str(&x).unwrap()).collect();
	
	if (p[0] >= p[2] && p[1] <= p[3]) || (p[0] <= p[2] && p[1] >= p[3]) {
		1
	}
	else {
		0
	}
}

fn processline2(ip:&String) -> i32 {
	let mut p : Vec<i32> = ip.split(&['-', ','][..]).map(|x| i32::from_str(&x).unwrap()).collect();
	
	if (p[1] < p[2] || p[0] > p[3]) {
		0
	}
	else {
		1
	}
}

fn part1(){
	println!("Hello world!");

	let mut score = 0;

	let lines:Vec<String> = read_lines("input4.txt").expect("Failing").map(|l| l.expect("Could not parse line"))
        .collect();
    let len = lines.len();
    for i in 0..len {
    	score += processline2(&lines[i]);
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