pub mod data;

use std::process::{Command, Stdio};
use std::io::{Write};
use itertools::{Itertools};


fn thruster_signal(data: &str, path: &str, phase: i32, signal: i32) -> i32 {
	let mut result = Command::new(path)
		.args(&[data])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn().expect("Unable to start child process");

	let child_input = result.stdin.as_mut().expect("Unable to capture input");

	write!(child_input, "{}\n{}", phase, signal).expect("Unable to write input");

	let output = result.wait_with_output().expect("Unable to capture result");

	String::from_utf8(output.stdout).unwrap().trim().parse::<i32>()
		.expect("Unable to parse output signal")
}


fn main() {
	let path = "../../05/intcode/target/release/intcode.exe";
	let data = data::data();

	let amplifier_count = 5;

	let mut input_signal = 0;
	let mut max = 0;
	let mut max_soln = None;

	for configuration in (0..amplifier_count).permutations(amplifier_count) {
		for &phase_setting in &configuration {
			input_signal = thruster_signal(data, path, phase_setting as i32, input_signal);
		}
		if input_signal > max {
			max = input_signal;
			max_soln = Some(configuration);
		}
		input_signal = 0;
	}

	if let Some(soln) = max_soln {
		println!("soln({:?}) = {:?}", max, soln);
	}
}
