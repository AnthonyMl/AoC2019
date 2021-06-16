use std::{collections::HashMap, io::{stdin}};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use num_bigint::{BigInt};

mod data;


#[derive(FromPrimitive, ToPrimitive, Debug)]
enum ParameterMode {
	Position  = 0,
	Immediate = 1,
	Relative  = 2,
}
use ParameterMode::*;
#[derive(FromPrimitive, ToPrimitive, Debug)]
enum AddressType {
	Src = 1,
	Dst = 2,
}
use AddressType::*;

#[derive(FromPrimitive, Debug)]
enum Opcode {
	Add           = 1,
	Multiply      = 2,
	Input         = 3,
	Output        = 4,
	JumpIfTrue    = 5,
	JumpIfFalse   = 6,
	LessThan      = 7,
	Equals        = 8,
	OffsetRelBase = 9,
	Quit          = 99,
}

fn parameter_count(opcode: &Opcode) -> u32 {
	match opcode {
		Opcode::Add           => 3,
		Opcode::Multiply      => 3,
		Opcode::Input         => 1,
		Opcode::Output        => 1,
		Opcode::JumpIfTrue    => 2,
		Opcode::JumpIfFalse   => 2,
		Opcode::LessThan      => 3,
		Opcode::Equals        => 3,
		Opcode::OffsetRelBase => 1,
		Opcode::Quit          => 0,
	}
}

struct Data(HashMap::<BigInt, BigInt>);

impl Data {
	fn new() -> Self {
		Data(HashMap::new())
	}

	fn read(&self, index: &BigInt) -> BigInt {
		if let Some(k) = self.0.get(index) {
			k.clone()
		} else {
			BigInt::zero()
		}
	}

	fn write(&mut self, index: BigInt, value: BigInt) {
		self.0.insert(index, value);
	}
}

// TODO: something about these unwraps
fn parse_instruction(input: &BigInt) -> (Vec<ParameterMode>, Opcode) {
	let opcode = Opcode::from_u8((input % BigInt::from(100)).to_u8().unwrap()).unwrap();

	let modes = (0..parameter_count(&opcode)).map(|i| {
		let shift = BigInt::from(100 * 10u128.pow(i));

		let pm = (input / &shift) % BigInt::from(10);

		ParameterMode::from_u8(pm.to_u8().unwrap()).unwrap()
	}).collect();
	(modes, opcode)
}

fn parse_bigint(data: &str) -> Option<BigInt> {
	BigInt::parse_bytes(data.as_bytes(), 10)
}

fn parse_args() -> Result<Vec<BigInt>, String> {
	// TODO: avoid this shuffling
	match data::data().split(',').map(|x| parse_bigint(x).ok_or("Parse failure")).collect() {
		Ok(args) => Ok(args),
		Err(e) => Err(e.to_string())
	}
}

fn main() {
	let mut data = {
		let mut data = Data::new();

		for (i, argument) in parse_args().unwrap().into_iter().enumerate() {
			data.write(BigInt::from(i), argument);
		}
		data
	};

	let mut instruction_pointer = BigInt::zero();
	let mut relative_base = BigInt::zero();
	loop {
		let (parameter_modes, opcode) = parse_instruction(&data.read(&instruction_pointer));
		instruction_pointer += 1;

		// TODO: clean this up somehow
		let mut get_params = |address_types: &[AddressType]|{
			fn parameters(
				data: &mut Data,
				instruction_pointer: &BigInt,
				relative_base: &BigInt,
				parameter_modes: &[ParameterMode],
				address_types: &[AddressType]
			) -> Vec<BigInt> {
				parameter_modes.iter().zip(address_types).enumerate().map(|(i, mode_and_type)|{
					match &mode_and_type {
						(Position, Src) => {
							data.read(&data.read(&(instruction_pointer + i)))
						},
						(Position, Dst) | (Immediate, Src) => {
							data.read(&(instruction_pointer + i))
						},
						(Immediate, Dst) => {
							panic!("Error: Immediate Mode Write Address")
						},
						(Relative, Src) => {
							data.read(&(relative_base + data.read(&(instruction_pointer + i))))
						},
						(Relative, Dst) => {
							relative_base + data.read(&(instruction_pointer + i))
						},
				}}).collect()
			}
			parameters(&mut data, &instruction_pointer, &relative_base, &parameter_modes, address_types)
		};
		match opcode {
			Opcode::Add => {
				let parameters = get_params(&[Src,Src,Dst]);
				data.write(parameters[2].clone(), &parameters[0] + &parameters[1]);
			},
			Opcode::Multiply => {
				let parameters = get_params(&[Src,Src,Dst]);
				data.write(parameters[2].clone(), &parameters[0] * &parameters[1]);
			},
			Opcode::Input => {
				let parameters = get_params(&[Dst]);
				let mut input = String::new();
				stdin().read_line(&mut input).expect("Unable to read user input");
				data.write(parameters[0].clone(), input.trim().parse().unwrap());
			},
			Opcode::Output => {
				let parameters = get_params(&[Src]);
				println!("{}", &parameters[0]);
			},
			Opcode::JumpIfTrue => {
				let parameters = get_params(&[Src,Src]);
				if parameters[0] != BigInt::zero() { instruction_pointer = &parameters[1] - parameter_modes.len() }
			},
			Opcode::JumpIfFalse => {
				let parameters = get_params(&[Src,Src]);
				if parameters[0] == BigInt::zero() { instruction_pointer = &parameters[1] - parameter_modes.len() }
			},
			Opcode::LessThan => {
				let parameters = get_params(&[Src,Src,Dst]);
				data.write(parameters[2].clone(), BigInt::from_u8((parameters[0] < parameters[1]) as u8).unwrap());
			},
			Opcode::Equals => {
				let parameters = get_params(&[Src,Src,Dst]);
				data.write(parameters[2].clone(), BigInt::from_u8((parameters[0] == parameters[1]) as u8).unwrap());
			},
			Opcode::OffsetRelBase => {
				let parameters = get_params(&[Src]);
				relative_base += &parameters[0];
			}
			Opcode::Quit => break,
		}
		instruction_pointer += parameter_modes.len();
	}
}

