extern crate intersection_detection;

use std::collections::{HashMap};
use std::cmp::{min};
use intersection_detection::*;






fn main() {
	let wires = data();

	let mut map = HashMap::new();

	traverse_wire(&mut map, &wires[0], |m, k, v| { m.insert(k, v); });

	let mut closest_intersection = isize::max_value();

	traverse_wire(&mut map, &wires[1], |m, k, v| {
		if let Some(other_steps) = m.get(&k) {
			closest_intersection = min(closest_intersection, v + other_steps);
		}
	});

	println!("{}", closest_intersection);
}
