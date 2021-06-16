extern crate computer_repair;

use computer_repair::*;


fn main() {
	let mut data = data::data();

	data[1] = 12;
	data[2] = 2;

	println!("{}", interpret(&mut data));
}
