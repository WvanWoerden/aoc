use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;
use std::cmp;

fn count_line(heights: &Vec<i32>, n:i32, start:i32, incj:i32) -> usize {
	let h:i32 = heights[start as usize];
	for j in 1..n {
		let index:usize = (start + j*incj) as usize;
		if heights[index] >= h {
			return j as usize
		}
	}
	(n-1) as usize
}

fn view_line(heights: &Vec<i32>, visible: &mut Vec<bool>, n:i32, start:i32, incj:i32) {
	let mut h:i32 = -1;
	for j in 0..n {
		let index:usize = (start + j*incj) as usize;
		if heights[index] > h {
			visible[index] = true;
			h = heights[index];
		}
	}
}

fn make_visible(heights: &Vec<i32>, visible: &mut Vec<bool>, n:i32, starti:i32, inci:i32, startj:i32, incj:i32) {
	for i in 0..n {
		let iindex = starti + i*inci + startj;
		view_line(&heights, visible, n, iindex, incj);
	}
}

fn processlines(lines:&Vec<String>) -> usize {

	let n = lines.len();
	let mut heights : Vec<i32> = vec![0;n*n];
	for i in 0..n {
		let digits:Vec<_> = lines[i].chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
		for j in 0..n {
			heights[i*n+j] = digits[j]; 
		}
	}

	let mut visible : Vec<bool> = vec![false;n*n];

	let nn = n as i32;

	// left to right
	make_visible(&heights, &mut visible, nn, 0, nn, 0, 1);
	// right to left
	make_visible(&heights, &mut visible, nn, 0, nn, nn-1, -1);
	// top to bottom
	make_visible(&heights, &mut visible, nn, 0, 1, 0, nn);
	// bottom to top
	make_visible(&heights, &mut visible, nn, 0, 1, (nn-1)*nn, -nn);

	visible.iter().map(|&x| x as usize).sum()
}

fn processlines2(lines:&Vec<String>) -> usize {
	let n = lines.len();
	let mut heights : Vec<i32> = vec![0;n*n];
	for i in 0..n {
		let digits:Vec<_> = lines[i].chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
		for j in 0..n {
			heights[i*n+j] = digits[j]; 
		}
	}
	let nn = n as i32;

	let mut mxval = 0;
	for i in 1..(n-1) {
		for j in 1..(n-1) {
			let start = (i*n+j) as i32;
			// left to right
			let a = count_line(&heights, (n-j) as i32, start, 1);
			// right to left
			let b = count_line(&heights, (j+1) as i32, start, -1);
			// top to bottom
			let c = count_line(&heights, (n-i) as i32, start, nn);
			// bottom to top
			let d = count_line(&heights, (i+1) as i32, start, -nn);
			println!("{} {} {} {}", a,b,c,d);
			mxval = cmp::max(mxval, a*b*c*d);
		}
	}
		
	mxval
}

fn main() {
	let lines:Vec<String> = read_to_string("input8.txt").unwrap().lines().map(|x| x.to_string()).collect();
    let res = processlines2(&lines);

	println!("{res}\n");	
}
