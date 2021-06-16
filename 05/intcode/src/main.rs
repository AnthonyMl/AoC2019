use std::io::{stdin};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive};


#[derive(FromPrimitive, ToPrimitive, Debug)]
enum ParameterMode {
	Position = 0,
	Immediate = 1,
}

#[derive(FromPrimitive, Debug)]
enum Opcode {
	Add         = 1,
	Multiply    = 2,
	Input       = 3,
	Output      = 4,
	JumpIfTrue  = 5,
	JumpIfFalse = 6,
	LessThan    = 7,
	Equals      = 8,
	Quit        = 99,
}

fn parameter_count(opcode: &Opcode) -> u32 {
	match opcode {
		Opcode::Add         => 2,
		Opcode::Multiply    => 2,
		Opcode::Input       => 0,
		Opcode::Output      => 1,
		Opcode::JumpIfTrue  => 2,
		Opcode::JumpIfFalse => 2,
		Opcode::LessThan    => 2,
		Opcode::Equals      => 2,
		Opcode::Quit        => 0,
	}
}

// TODO: make an enum containing typed parameters

// TODO: try intoiter
fn parse_instruction(input: i32) -> (Vec<ParameterMode>, Opcode) {
	let opcode = Opcode::from_i32(input % 100).expect("Invalid Opcode");
	let modes = (0..parameter_count(&opcode))
		.map(|i|
			ParameterMode::from_i32(1 & (input / (100 * 10i32.pow(i))))
				.expect("Invalid Parameter Mode"))
		.collect();
	(modes, opcode)
}

fn parameters(parameter_modes: &[ParameterMode], data: &[i32], ip: usize) -> Vec<i32> {
	parameter_modes.iter().enumerate().map(|(i, x)| {
		match x {
			ParameterMode::Position => data[data[ip + i] as usize],
			ParameterMode::Immediate => data[ip + i],
		}
	}).collect()
}

fn parse_data(input: &str) -> Result<Vec<i32>, String> {
	if let Ok(data) = input.trim().split(',').map(|x|x.parse::<i32>()).collect() {
		return Ok(data)
	}
	Err(format!("Unable to parse program input: {}", input))
}


fn main() {
	let mut data = match std::env::args().nth(1) {
		Some(input) => {
			parse_data(&input).unwrap()
		},
		None => {
			parse_data(include_str!("../data/data.txt")).unwrap()
		}
	};

	let mut ip = 0;
	loop {
		let (parameter_modes, opcode) = parse_instruction(data[ip]);
		ip += 1;
		let parameters = parameters(&parameter_modes, &data, ip);
		ip += parameters.len();

		match opcode {
			Opcode::Add => {
				let dst = data[ip] as usize;
				data[dst] = parameters[0] + parameters[1];
				ip += 1;
			},
			Opcode::Multiply => {
				let dst = data[ip] as usize;
				data[dst] = parameters[0] * parameters[1];
				ip += 1;
			},
			Opcode::Input => {
				let dst = data[ip] as usize;
				let mut input_buffer = String::new();
				stdin().read_line(&mut input_buffer).expect("Unable to read user input");
				data[dst] = input_buffer.trim().parse::<i32>().expect(&format!("Unable to read user integer({}):", input_buffer));
				ip += 1;
			},
			Opcode::Output => {
				println!("{}", parameters[0]);
			},
			Opcode::JumpIfTrue => {
				if parameters[0] != 0 { ip = parameters[1] as usize }
			},
			Opcode::JumpIfFalse => {
				if parameters[0] == 0 { ip = parameters[1] as usize }
			},
			Opcode::LessThan => {
				let dst = data[ip] as usize;
				data[dst] = (parameters[0] < parameters[1]) as i32;
				ip += 1;
			},
			Opcode::Equals => {
				let dst = data[ip] as usize;
				data[dst] = (parameters[0] == parameters[1]) as i32;
				ip += 1;
			},
			Opcode::Quit => break,
		}
	}
}

