use std::fs::{read_to_string};
use std::collections::HashSet;
// use std::str::{parse};
use std::cmp::{min,max};

fn processlines(lines:&Vec<String>) -> usize {

	let m = lines.len();
	let rowy = 2000000;
	// let rowy = 10;

	let mut beacons = HashSet::new();
	let mut ints = Vec::new(); 
	for l in lines {
		let xyxy : Vec<_> = l.split(' ').map(str::parse::<i32>).map(|x| x.unwrap()).collect();
		let (sx,sy,bx,by) = (xyxy[0], xyxy[1], xyxy[2], xyxy[3]);


		if by == rowy {
			beacons.insert(bx);
		}

		let dist = (sx-bx).abs() + (sy-by).abs();
		println!("Sensor at {} {} at dist {}", sx, sy, dist);

		let ydist = (sy-rowy).abs();
		if ydist <= dist {
			let mut xmin = sx - (dist-ydist); 
			let mut xmax = sx + (dist-ydist); //inclusive

			// merge interval [xmin,xmax] into ints
			println!("Merge {} {} into {:?}", xmin, xmax, ints);
			let mut tmp_ints = Vec::new();
			while ints.len() > 0 {
				// println!("{} {} {:?}", xmin, xmax, ints);

				let int : (i32,i32) = ints.pop().unwrap();

				// do they overlap?
				if int.0 <= xmax && int.1 >= xmin  {
					// merge them
					// println!("Merging {} {} {} {}", int.0, int.1, xmin, xmax);
					xmin = min(int.0, xmin);
					xmax = max(int.1, xmax);
				} else {
					tmp_ints.push(int);
				}
			}
			tmp_ints.push((xmin, xmax));
			ints = tmp_ints;
			println!("Result: {:?}", ints);
		}
	}

	println!("{:?}", ints);
	println!("{:?}", beacons);


	// count total size of ints minus beacons
	let sm:usize = ints.iter().map(|x| (x.1-x.0+1) as usize).sum();
	sm - beacons.len()
}

fn processlines2(lines:&Vec<String>) -> usize {

	let m = lines.len();
	
	let mut rowy = 0;

	while <= 4000000 {
		let mut ints = Vec::new(); 
		for l in lines {
			let xyxy : Vec<_> = l.split(' ').map(str::parse::<i32>).map(|x| x.unwrap()).collect();
			let (sx,sy,bx,by) = (xyxy[0], xyxy[1], xyxy[2], xyxy[3]);

			let dist = (sx-bx).abs() + (sy-by).abs();

			let ydist = (sy-rowy).abs();
			if ydist <= dist {
				let mut xmin = sx - (dist-ydist); 
				let mut xmax = sx + (dist-ydist); //inclusive

				// merge interval [xmin,xmax] into ints
				let mut tmp_ints = Vec::new();
				while ints.len() > 0 {
					// println!("{} {} {:?}", xmin, xmax, ints);

					let int : (i32,i32) = ints.pop().unwrap();

					// do they overlap?
					if int.0 <= xmax && int.1 >= xmin  {
						// merge them
						// println!("Merging {} {} {} {}", int.0, int.1, xmin, xmax);
						xmin = min(int.0, xmin);
						xmax = max(int.1, xmax);
					} else {
						tmp_ints.push(int);
					}
				}
				tmp_ints.push((xmin, xmax));
				ints = tmp_ints;
			}
		}

		// lets just assume answer is not on border 
		if ints.len() > 1 {
			println!("{} {:?}", rowy, ints);
		}

		//
		rowy = rowy - min(ints[0].0, ints[0].1 - 4000000);

	}


	// // count total size of ints minus beacons
	// let sm:usize = ints.iter().map(|x| (x.1-x.0+1) as usize).sum();
	// sm - beacons.len()
	m
}

fn main() {
	let lines:Vec<String> = read_to_string("input15_clean.txt").unwrap().lines().map(|x| x.to_string()).collect();
    let res = processlines2(&lines);

	println!("{res}\n");	
}
