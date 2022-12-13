use std::fs::{read_to_string};
// use std::collections::VecDeque;
use std::str::FromStr;

// do operation and divide by 3
fn operation(s:&String, old: usize) -> usize {
	let ss : Vec<_> = s.split(' ').collect();
	let a = match ss[0] {
		"old" => old,
		&_ => usize::from_str(ss[0]).unwrap(),
	};
	let b = match ss[2] {
		"old" => old,
		&_ => usize::from_str(ss[2]).unwrap(),
	};
	match ss[1] {
		"+" => (a+b)/3,
		"*" => (a*b)/3,
		&_ => todo!()
	}
}

// do operation and divide by 3
fn operation2(s:&String, old: usize, modular:usize) -> usize {
	let ss : Vec<_> = s.split(' ').collect();
	let a = match ss[0] {
		"old" => old,
		&_ => usize::from_str(ss[0]).unwrap(),
	};
	let b = match ss[2] {
		"old" => old,
		&_ => usize::from_str(ss[2]).unwrap(),
	};
	match ss[1] {
		"+" => (a+b)%modular,
		"*" => (a*b)%modular,
		&_ => todo!()
	}
}


fn processlines(lines:&Vec<String>) -> usize {

	let m = (lines.len()+6) / 7;

	let mut items = vec![Vec::new(); m];
	let mut ops : Vec<String> = vec!["old + 5".to_string(); m];
	let mut divBy = vec![0;m];
	let mut toTrue = vec![0;m];
	let mut toFalse = vec![0;m];
	let mut cntItems = vec![0;m];

	for i in 0..m {
		let (_, its) = lines[7*i+1].split_once(": ").unwrap();
		items[i] = its.split(", ").map(|x| usize::from_str(&x).unwrap()).collect();
		let (_, op) = lines[7*i+2].split_once(" = ").unwrap();
		ops[i] = op.to_string();
		let (_, div) = lines[7*i+3].split_once("by ").unwrap();
		divBy[i] = usize::from_str(div).unwrap();
		let (_, t) = lines[7*i+4].split_once("monkey ").unwrap();
		toTrue[i] = usize::from_str(t).unwrap();
		let (_, f) = lines[7*i+5].split_once("monkey ").unwrap();
		toFalse[i] = usize::from_str(f).unwrap();
	}

	for _ in 0..20 {
		for i in 0..m {
			cntItems[i] += items[i].len();
			let its = items[i].clone();
			// println!("{}", items[i].len());
			for x in its.iter() {
				// do computation
				let res = operation(&ops[i], *x);
				if res%divBy[i] == 0 {
					items[toTrue[i]].push(res);
				} else {
					items[toFalse[i]].push(res);
				}
			}
			items[i].clear();
		}
	}

	cntItems.sort();
	cntItems[m-1] * cntItems[m-2]
}

fn processlines2(lines:&Vec<String>) -> usize {

	let m = (lines.len()+6) / 7;

	let mut items = vec![Vec::new(); m];
	let mut ops : Vec<String> = vec!["old + 5".to_string(); m];
	let mut divBy = vec![0;m];
	let mut toTrue = vec![0;m];
	let mut toFalse = vec![0;m];
	let mut cntItems = vec![0;m];

	for i in 0..m {
		let (_, its) = lines[7*i+1].split_once(": ").unwrap();
		items[i] = its.split(", ").map(|x| usize::from_str(&x).unwrap()).collect();
		let (_, op) = lines[7*i+2].split_once(" = ").unwrap();
		ops[i] = op.to_string();
		let (_, div) = lines[7*i+3].split_once("by ").unwrap();
		divBy[i] = usize::from_str(div).unwrap();
		let (_, t) = lines[7*i+4].split_once("monkey ").unwrap();
		toTrue[i] = usize::from_str(t).unwrap();
		let (_, f) = lines[7*i+5].split_once("monkey ").unwrap();
		toFalse[i] = usize::from_str(f).unwrap();
	}

	let modular = divBy.iter().product();

	for _ in 0..10000 {
		for i in 0..m {
			cntItems[i] += items[i].len();
			let its = items[i].clone();
			// println!("{}", items[i].len());
			for x in its.iter() {
				// do computation
				let res = operation2(&ops[i], *x, modular);
				if res%divBy[i] == 0 {
					items[toTrue[i]].push(res);
				} else {
					items[toFalse[i]].push(res);
				}
			}
			items[i].clear();
		}
	}

	cntItems.sort();
	cntItems[m-1] * cntItems[m-2]
}


fn main() {
	let lines:Vec<String> = read_to_string("input11.txt").unwrap().lines().map(|x| x.to_string()).collect();
    let res = processlines2(&lines);

	println!("{res}\n");	
}
