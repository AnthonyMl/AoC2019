extern crate computer_repair;

use computer_repair::*;


fn main() {
	let data = data::data();

	let upper_bound = std::cmp::min(data.len(), 100);
	for noun in 0..upper_bound {
		for verb in 0..upper_bound {
			let mut local = data.clone();
			local[1] = noun;
			local[2] = verb;
			if interpret(&mut local) == 19690720 {
				println!("{}", 100 * noun + verb);
				return
			}
		}
	}
}
