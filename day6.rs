use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::str::FromStr;

fn processline(line:String, markersize:usize) -> usize {
	let m = line.len();
	for i in 0..m-markersize+1 {
		let mut x = &line[i..i+markersize];
		println!("{}", x);
		let mut y:Vec<char> = x.chars().collect();
		y.sort();
		let mut distinct = true;
		for j in 0..markersize-1 {
			if y[j] == y[j+1] {
				distinct = false;
			}
		}
		if distinct {
			return i+markersize
		}
	}
	m
}


fn part1(){
	println!("Hello world!");

	let lines:Vec<String> = read_lines("input6.txt").expect("Failing").map(|l| l.expect("Could not parse line"))
        .collect();

    let res = processline(lines[0].clone(), 4);
	println!("{res}\n");
	let res2 = processline(lines[0].clone(), 14);
	println!("{res2}\n");
}

fn main() {
	part1();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}