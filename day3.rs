use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn char_topriority(c:char) -> i32 {
	let i = c as i32;
	match i {
		65..=90 => i-38,
		_ => i-96,
	}
}

fn priority(ip:String) -> i32 {
	let mut a = HashSet::new();
	let mut b = HashSet::new();
	let len = ip.chars().count();
	for (i,c) in ip.chars().enumerate() {
		if i < len/2 {
			a.insert(char_topriority(c));
		}
		else {
			b.insert(char_topriority(c));
		}
	}
	let mut inter = a.intersection(&b);



	*inter.nth(0).unwrap()
}

fn inter3(ip1:&String, ip2:&String, ip3:&String) -> i32 {
	let mut a1:HashSet<i32> = ip1.chars().map(|c| char_topriority(c)).collect();
	let mut a2:HashSet<i32> = ip2.chars().map(|c| char_topriority(c)).collect();
	let mut a3:HashSet<i32> = ip3.chars().map(|c| char_topriority(c)).collect();
	let inter12: HashSet<i32> = a1.intersection(&a2).cloned().collect();
	println!("{}", inter12.len());
	let mut inter = inter12.intersection(&a3);
	*inter.nth(0).unwrap()
}


fn part1(){
	println!("Hello world!");

	let mut score = 0;

	let lines:Vec<String> = read_lines("input3.txt").expect("Failing").map(|l| l.expect("Could not parse line"))
        .collect();
    let len = lines.len()/3;
    for i in 0..len {
    	score += inter3(&lines[3*i], &lines[3*i+1], &lines[3*i+2]);
    }

	// if let Ok(lines) = read_lines("input3.txt") {
	// 	for line in lines {
	// 		if let Ok(ip) = line {
	// 			score += priority(ip);
	// 			println!("{score}\n");
	// 		}
	// 	}
	// }
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