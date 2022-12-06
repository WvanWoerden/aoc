use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::str::FromStr;

fn processlines(lines:&Vec<String>) -> String {
	let n = 9;
	let m = lines.len();

	let mut stacks = vec![Vec::<char>::new(); n];

	for i in 0..8 {
		let chrs:Vec<char> = lines[i].chars().collect();
		for j in 0..n {
			let c = chrs[1+4*j];
			if c!=' ' {
				stacks[j].push(c);
			}
		}
	}
	for j in 0..n {
		stacks[j].reverse();
	}

	// process input
	for i in 10..m {
		let words = lines[i].split(' ').collect::<Vec<_>>();
		let nr:usize = usize::from_str(&words[1]).unwrap();
		let from:usize = usize::from_str(&words[3]).unwrap()-1;
		let to:usize = usize::from_str(&words[5]).unwrap()-1;
		println!("{} {} {}", nr, from, to);
		for _r in 0..nr {
			if let Some(c) = stacks[from].pop() {
				stacks[to].push(c);
			}
		}
	}

	stacks.iter().map(|vc| vc.last().unwrap()).collect()
}

fn processlines2(lines:&Vec<String>) -> String {
	let n = 9;
	let m = lines.len();

	let mut stacks = vec![Vec::<char>::new(); n];

	for i in 0..8 {
		let chrs:Vec<char> = lines[i].chars().collect();
		for j in 0..n {
			let c = chrs[1+4*j];
			if c!=' ' {
				stacks[j].push(c);
			}
		}
	}
	for j in 0..n {
		stacks[j].reverse();
	}

	// process input
	for i in 10..m {
		let words = lines[i].split(' ').collect::<Vec<_>>();
		let nr:usize = usize::from_str(&words[1]).unwrap();
		let from:usize = usize::from_str(&words[3]).unwrap()-1;
		let to:usize = usize::from_str(&words[5]).unwrap()-1;
		println!("{} {} {}", nr, from, to);

		let l = stacks[from].len();
		for r in l-nr..l {
			let c : char = stacks[from][r];
			stacks[to].push(c);
		}
		// stacks[to].extend_from_slice(&stacks[from][(l-nr)..l]);
		stacks[from].resize(l-nr, ' ');

		// for r in 0..nr {
		// 	if let Some(c) = stacks[from].pop() {
		// 		stacks[to].push(c);
		// 	}
		// }
	}

	stacks.iter().map(|vc| vc.last().unwrap()).collect()
}

fn part1(){
	println!("Hello world!");

	let lines:Vec<String> = read_lines("input5.txt").expect("Failing").map(|l| l.expect("Could not parse line"))
        .collect();
    let res = processlines2(&lines);

	println!("{res}\n");
}

fn main() {
	part1();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}