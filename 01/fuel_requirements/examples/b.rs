extern crate fuel_requirements;

use fuel_requirements::*;


fn main() {
	let sum: isize =
		data::data().into_iter()
		.flat_map(Fuel)
		.sum();

	println!("{}", sum);
}
