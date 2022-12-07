use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

struct Dir {
	// name : String,
	dirs : HashMap<String, usize>,
	files : HashMap<String, usize>
}

fn cntsizes(alldirs : &Vec<Dir>, res: &mut Vec<usize>, index:usize) {

	let mut total:usize = alldirs[index].files.values().sum();

	for ind in alldirs[index].dirs.values() {
		cntsizes(&alldirs, res, *ind);
		total += res[*ind];
	}
	res[index] = total;
}

fn processlines(lines:&Vec<String>) -> usize {

	let m = lines.len();


	let root = Dir{dirs: HashMap::new(), files: HashMap::new()};
	let mut alldirs : Vec<Dir> = vec![root];

	let mut prefix : Vec<String> = vec!["".to_string()];
	let mut curdirs : Vec<usize> = vec![0];
	for i in 0..m {
		let x: Vec<_> = lines[i].split(' ').collect();
		if x[0] == "$" {
			if x[1] == "ls" {
				continue;
			}
			else if x[1] == "cd" {
				if x[2] == "/" {
					// reset to root
					curdirs.drain(1..);
					prefix.drain(1..);
				}
				else if x[2] == ".." {
					curdirs.pop();
					prefix.pop();
				}
				else {
					if let Some(index) = curdirs.last() {
						let name = x[2].to_string();
						
						// check if dir exists
					    if !alldirs[*index].dirs.contains_key(&name) {
					    	let n = alldirs.len();
					    	alldirs.push(Dir{dirs: HashMap::new(), files: HashMap::new()});
					    	alldirs[*index].dirs.insert(name.clone(), n);
					    }
						
						// move
						if let Some(x) = alldirs[*index].dirs.get(&name) {
					    	curdirs.push(*x);
						}
					    prefix.push(name);
					}
				}
			} 
		} 
		else if x[0] == "dir" {
			if let Some(index) = curdirs.last() {
				// add dir if it doesn't exist
				let name = x[1].to_string();
			    if !alldirs[*index].dirs.contains_key(&name) {
			    	let n = alldirs.len();
			    	alldirs.push(Dir{dirs: HashMap::new(), files: HashMap::new()});
			    	alldirs[*index].dirs.insert(name.clone(), n);
			    }
			}
		} else { //file
			let size:usize = usize::from_str(x[0]).unwrap();
			let name = x[1].to_string();
			if let Some(index) = curdirs.last() {
				if !alldirs[*index].files.contains_key(&name) {
			    	alldirs[*index].files.insert(name.clone(), size);
			    }
			}
		}
	}

	let mut res : Vec<usize> = vec![0; alldirs.len()];
	cntsizes(&alldirs, &mut res, 0);
	
	// part 1
	// let mut totalsmall = 0;
	// for i in 0..alldirs.len() {
	// 	if res[i] <= 100000 {
	// 		totalsmall += res[i];
	// 	}
	// }
	// totalsmall

	// part 2
	let needed =  res[0] - 40000000;
	let mut smallestval = 70000001;
	for i in 0..alldirs.len() {
		if res[i] >= needed  {
			if res[i] < smallestval {
				smallestval = res[i];
			}
		}
	}
	smallestval
}

// fn processlines2(lines:&Vec<String>) -> String {

// }

fn part1(){
	println!("Hello world!");

	let lines:Vec<String> = read_lines("input7.txt").expect("Failing").map(|l| l.expect("Could not parse line"))
        .collect();
    let res = processlines(&lines);

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