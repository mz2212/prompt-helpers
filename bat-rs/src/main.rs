use std::fs::File;
use std::io::prelude::*;

fn main() {
	let mut file = File::open("/sys/class/power_supply/BAT0/capacity").unwrap();
	let mut string = String::new();
	file.read_to_string(&mut string).unwrap();
	let _ = string.pop();
	
	let charge = string.parse::<i32>().unwrap();

	if charge <= 25 {
		print!("%B%F{{red}}{}%b%f ", charge);
	} else if charge <= 50 {
		print!("%B%F{{yellow}}{}%b%f ", charge);
	}
}
