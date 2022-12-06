use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn part1(){
	println!("Hello world!");

	let mut val = 0;
	let mut vec = Vec::new();

	if let Ok(lines) = read_lines("input1_1.txt") {
		for line in lines {
			if let Ok(ip) = line {
				if ip=="" {
					vec.push(val);
					val = 0;
				} else {
					val += i32::from_str(&ip).unwrap();
				}
			}
		}
	}
	vec.push(val);
	vec.sort_by(|a, b| b.cmp(a));
	let top3 = vec[0] + vec[1] + vec[2];

	println!("Mxval: {top3}\n");
}

fn main() {
	part1();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}