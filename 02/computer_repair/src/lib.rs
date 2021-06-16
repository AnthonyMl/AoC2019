pub mod data;


pub fn interpret(data: &mut [usize]) -> usize {
	let mut i = 0;
	while let op @ (1 | 2) = data[i] {
		let a   = data[data[i+1]];
		let b   = data[data[i+2]];
		let dst =      data[i+3];
		data[dst] =
			if op == 1 { a + b }
			else       { a * b };
		i += 4;
	}
	data[0]
}
