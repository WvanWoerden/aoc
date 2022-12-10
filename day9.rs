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

fn processlines(lines:&Vec<String>) -> usize {

	let n = lines.len();

	let mut H = Point{x:0, y:0};
	let mut T = Point{x:0, y:0};
	let mut S = HashSet::new();
	S.insert(T.clone());

	for s in lines {
		let p : Vec<_> = s.split(' ').collect();
		for _ in 0..usize::from_str(p[1]).unwrap() {

			match p[0] {
				"R" => H.x+=1,
				"L" => H.x-=1,
				"U" => H.y+=1,
				"D" => H.y-=1,
				&_ => todo!()
			}

			// update T
			let xd = H.x - T.x;
			let yd = H.y - T.y;
			if(xd.abs() > 1 || yd.abs() > 1) {
				if xd != 0 {
					T.x += xd/xd.abs();
				}
				if yd != 0 {
					T.y += yd/yd.abs();
				}
			}

			// push T to set
			S.insert(T.clone());

		}

	}

	S.len()
}


fn processlines2(lines:&Vec<String>) -> usize {

	let n = lines.len();

	let mut T = vec![Point{x:0, y:0}; 10];
	let mut S = HashSet::new();
	S.insert(T[9].clone());

	for s in lines {
		let p : Vec<_> = s.split(' ').collect();
		for _ in 0..usize::from_str(p[1]).unwrap() {

			match p[0] {
				"R" => T[0].x+=1,
				"L" => T[0].x-=1,
				"U" => T[0].y+=1,
				"D" => T[0].y-=1,
				&_ => todo!()
			}

			// update tails
			for i in 0..9 {
				let xd = T[i].x - T[i+1].x;
				let yd = T[i].y - T[i+1].y;
				if(xd.abs() > 1 || yd.abs() > 1) {
					if xd != 0 {
						T[i+1].x += xd/xd.abs();
					}
					if yd != 0 {
						T[i+1].y += yd/yd.abs();
					}
				}
			}

			// push T to set
			S.insert(T[9].clone());

		}

	}

	S.len()
}


fn main() {
	let lines:Vec<String> = read_to_string("input9.txt").unwrap().lines().map(|x| x.to_string()).collect();
    let res = processlines2(&lines);

	println!("{res}\n");	
}
