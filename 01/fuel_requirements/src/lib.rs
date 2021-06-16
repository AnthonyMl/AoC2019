pub mod data;

pub struct Fuel(pub isize);

impl std::convert::From<isize> for Fuel {
	fn from(fuel: isize) -> Self { Fuel(fuel) }
}

impl Iterator for Fuel {
	type Item = isize;

	fn next(&mut self) -> Option<isize> {
		let fuel = required_fuel(self.0);
		if let Some(k) = fuel { self.0 = k }
		fuel
	}
}

pub fn required_fuel(mass: isize) -> Option<isize> {
	let f = (mass / 3) - 2;

	if f <= 0 { None }
	else { Some(f) }
}
