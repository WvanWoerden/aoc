use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::str::FromStr;
use std::cmp;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
	x: i32,
	y: i32
}

fn processlines(lines:&Vec<String>) -> i32 {

	let mut c:usize = 0;
	let mut X:i32 = 1;
	let mut total = 0;

	let mut done = vec![false; 6];

	let mut crt: Vec<char> = vec!['.'; 240];

	for s in lines {
		let p : Vec<_> = s.split(' ').collect();
		
		let cycles = match p[0] {
			"noop" => 1,
			"addx" => 2,
			&_ => todo!(),
		};

		for i in 0..cycles {

			// part 2
			if (X-((c%40) as i32)).abs() <= 1 {
				crt[c] = '#';
			}

			c += 1;

			// part 1
			if c >= 20 && !done[(c-20)/40] {
				done[(c-20)/40] = true;
				// println!("{} {}", c, X);
				total += ((c-c%20) as i32)*X;
			}

		}		

		// end, add
		if p[0] == "addx" {
			// println!("{} adding {}", c, i32::from_str(p[1]).unwrap());
			X += i32::from_str(p[1]).unwrap();
		}
	}

	for i in 0..6 {
		for j in 0..40 {
			print!("{}", crt[i*40+j]);
		}
		println!("");
	}

	total
}


fn main() {
	let lines:Vec<String> = read_to_string("input10.txt").unwrap().lines().map(|x| x.to_string()).collect();
    let res = processlines(&lines);

	println!("{res}\n");	
}
