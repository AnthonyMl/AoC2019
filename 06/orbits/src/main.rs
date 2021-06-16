pub mod data;

use std::collections::{BTreeMap, HashMap, HashSet, LinkedList};


// TODO: try doing recursively
//
fn path(node: &str, target: &str, children: &HashMap<String, Vec<String>>) -> Option<LinkedList<String>> {
	if node == target {
		let mut l = LinkedList::new();
		l.push_back(target.to_string());
		return Some(l)
	}
	for child in children.get(node).unwrap_or(&Vec::new()) {
		if let Some(mut l) = path(child, target, children) {
			l.push_back(node.to_string());
			return Some(l)
		}
	}
	None
}

fn path_length(a: &LinkedList<String>, b: &LinkedList<String>) -> usize {
	let prefix_length =
		a.iter().rev()
		.zip(b.iter().rev())
		.take_while(|(a,b)|a == b)
		.count();

	a.len() + b.len() - 2 * prefix_length - 2
}


fn main() {
	let mut orbits = {
		let mut v = data::data();
		v.sort_unstable();

		let mut os: HashMap<String, Vec<String>> = HashMap::new();
		let mut values = Vec::new();
		let mut key = v[0].base.clone();
		for orbit in &v {
			if orbit.base != key {
				let mut tmp = Vec::new();
				std::mem::swap(&mut values, &mut tmp);
				os.insert(key, tmp);
				key = orbit.base.clone();
			}
			values.push(orbit.orbiter.clone())
		}
		os.insert(key, values);
		os
	};

	match std::env::args().nth(1).as_deref() {
		Some("A") | Some("a") => {
			let mut sum = 0;

			let mut working_set = {
				let mut ws = BTreeMap::new();
				let mut root_set = HashSet::new();
				root_set.insert("COM".to_string());
				ws.insert(1, root_set);
				ws
			};

			while let Some(item) = working_set.iter().next() {
				let depth = *item.0;

				let bases = working_set.remove(&depth).unwrap();

				for base in bases {
					if let Some(orbiters) = orbits.remove(&base) {
						sum += depth * orbiters.len();
						working_set.entry(depth+1).or_default().extend(orbiters.into_iter());
					}
				}
			}

			println!("(A) Total orbits = {}", sum);
		},
		Some("B") | Some("b") => {
			if let Some(p) = path("COM", "SAN", &orbits) {
				if let Some(q) = path("COM", "YOU", &orbits) {
					println!("(B) Minimum orbit transfers = {}", path_length(&p, &q));
				}
			}
		},
		Some(_) | None => {
			eprint!("Invalid input: provide A or B");
		}
	};

}
