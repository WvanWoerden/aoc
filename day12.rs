use std::fs::{read_to_string};
use std::collections::VecDeque;
use std::str::FromStr;
use std::cmp::min;

// fn flow(mp: &Vec<Vec<usize>>, flow: &mut Vec<Vec<usize>>, i:usize, j:usize, f:usize) {
// 	if f < flow[i][j] {
// 		flow[i][j] = f;

// 		let h = mp.len();
// 		let w = mp[0].len();

// 		if i > 0 {

// 		}

// 	}


// }


fn processlines(lines:&Vec<String>) -> usize {

	let h = lines.len();
	let w = lines[0].len();
	let mut mp = vec![vec![0;w];h];
	let mut dist = vec![vec![2*h*w;w];h];

	for i in 0..h {
		mp[i] = lines[i].chars().map(|x| x as usize).collect();
	}


	// println!("{}", lines[20]);
	mp[20][0] = 'a' as usize; // start
	mp[20][112] = 'z' as usize; // end
	// mp[0][0] = 'a' as usize; // start
	// mp[2][5] = 'z' as usize; // end

	// floodfill
	let mut q:VecDeque<(usize, usize, usize)> = VecDeque::new();
	q.push_back((20, 0, 0));
	// q.push_back((0, 0, 0));

	while let Some((ii, jj, ff)) = q.pop_front() {
		if ff < dist[ii][jj] {
			dist[ii][jj] = ff;

			let height = mp[ii][jj];
			if ii > 0 && mp[ii-1][jj] <= height+1 {
				q.push_back((ii-1, jj, ff+1));
			}
			if ii < h-1 && mp[ii+1][jj] <= height+1 {
				q.push_back((ii+1, jj, ff+1));
			}
			if jj > 0 && mp[ii][jj-1] <= height+1 {
				q.push_back((ii, jj-1, ff+1));
			}
			if jj < w-1 && mp[ii][jj+1] <= height+1 {
				q.push_back((ii, jj+1, ff+1));
			}
		}
	}

	dist[20][112]
	// dist[2][5]
}

fn processlines2(lines:&Vec<String>) -> usize {

	let h = lines.len();
	let w = lines[0].len();
	let mut mp = vec![vec![0;w];h];
	let mut dist = vec![vec![2*h*w;w];h];

	for i in 0..h {
		mp[i] = lines[i].chars().map(|x| x as usize).collect();
	}


	// println!("{}", lines[20]);
	mp[20][0] = 'a' as usize; // start
	mp[20][112] = 'z' as usize; // end
	// mp[0][0] = 'a' as usize; // start
	// mp[2][5] = 'z' as usize; // end

	// floodfill starting at E
	let mut q:VecDeque<(usize, usize, usize)> = VecDeque::new();
	q.push_back((20, 112, 0));

	while let Some((ii, jj, ff)) = q.pop_front() {
		if ff < dist[ii][jj] {
			dist[ii][jj] = ff;

			let height = mp[ii][jj];
			if ii > 0 && mp[ii-1][jj] +1>= height {
				q.push_back((ii-1, jj, ff+1));
			}
			if ii < h-1 && mp[ii+1][jj] +1>= height {
				q.push_back((ii+1, jj, ff+1));
			}
			if jj > 0 && mp[ii][jj-1] +1 >= height {
				q.push_back((ii, jj-1, ff+1));
			}
			if jj < w-1 && mp[ii][jj+1] +1 >= height {
				q.push_back((ii, jj+1, ff+1));
			}
		}
	}

	let mut mndist = w*h;
	for i in 0..h {
		for j in 0..w {
			if mp[i][j] == ('a' as usize) {
				mndist = min(mndist, dist[i][j]);
			}
		}
	}

	mndist
}


fn main() {
	let lines:Vec<String> = read_to_string("input12.txt").unwrap().lines().map(|x| x.to_string()).collect();
    let res = processlines2(&lines);

	println!("{res}\n");	
}
