pub mod data;

use std::io::{Result};
use std::process::{Stdio};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use itertools::{Itertools};


struct Amplifier {
	_child: Child,
	input: ChildStdin,
	output: BufReader<ChildStdout>,
}


impl Amplifier {
	fn path() -> String {
		"../../05/intcode/target/release/intcode.exe".to_string()
	}
	fn data() -> String {
		data::data().to_string()
	}

	async fn new_amplifiers(phases: &[i32]) -> Result<Vec<Amplifier>> {
		let mut amplifiers: Vec<Amplifier> = Vec::new();

		for phase in phases {
			let mut child = Command::new(Amplifier::path())
				.args(&[Amplifier::data()])
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.spawn().expect("Unable to start child process");

			let mut input = child.stdin.take().unwrap();

			let buf = format!("{}\n", *phase);
			input.write_all(buf.as_bytes()).await?;

			let output = child.stdout.take().expect("Unable to capture child output");
			let output = BufReader::new(output);

			amplifiers.push(Amplifier { _child: child, input, output });
		}
		Ok(amplifiers)
 	}

	async fn amplify(&mut self, signal: i32) -> Result<Option<i32>> {
		let input_buf = format!("{}\n", signal);
		self.input.write_all(input_buf.as_bytes()).await?;

		let mut output_buf = String::new();

		let bytes_read = self.output.read_line(&mut output_buf).await?;

		if bytes_read == 0 { return Ok(None) }

		match output_buf.trim().parse::<i32>() {
			Ok(integer) => { Ok(Some(integer)) }
			Err(e) => { panic!("Not an integer. Bytes read {}.\nErr({})", bytes_read, e) }
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	const AMPLIFIER_COUNT: usize = 5;
	let mut input_signal = 0;
	let mut max = 0;
	let mut max_soln = vec![-1; AMPLIFIER_COUNT];

	for phase_configuration in (5..10).permutations(AMPLIFIER_COUNT) {
		let mut amplifiers = Amplifier::new_amplifiers(&phase_configuration).await?;

		'outer: loop {
			for amplifier in &mut amplifiers {
				match amplifier.amplify(input_signal).await {
					Ok(Some(signal)) => { input_signal = signal },
					Ok(None) | Err(_) => { break 'outer },
				}
			}
		}

		if input_signal > max {
			max = input_signal;
			max_soln = phase_configuration;
		}
		input_signal = 0;
	}
	println!("soln({:?}) = {:?}", max, max_soln);
	Ok(())
}
