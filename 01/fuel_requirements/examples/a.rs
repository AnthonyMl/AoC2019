extern crate fuel_requirements;

use fuel_requirements::*;


fn main() {
	let sum: isize =
		data::data().into_iter()
		.map(required_fuel)
		.map(|s| s.unwrap_or(0))
		.sum();

	println!("{}", sum);
}
