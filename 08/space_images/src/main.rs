extern crate image;
extern crate itertools;

pub mod data;

use itertools::Itertools;


#[repr(u8)]
enum Color {
	Black = 33,
	White = 166,
	Invalid = 255,
}

fn main() {
	const WIDTH : usize = 25;
	const HEIGHT: usize = 6;
	const FILENAME: &str = "decoded_password.png";
	let data = data::data();

	let layers = ||{ data.chunks(WIDTH).into_iter().chunks(HEIGHT) };

	// Part 1

	let (ones, twos) = {
		let min_layer = {
			let layers = layers();

			let zeroes = layers.into_iter().map(|layer| layer.flatten().fold(0, |acc,x| acc + if *x == '0' { 1 } else { 0 }));

			zeroes.enumerate().fold((0, std::usize::MAX), |(min_layer, min_zeroes), (layer, count)|
				if count < min_zeroes { (layer, count) } else { (min_layer, min_zeroes)
			}).0
		};

		let layers = layers();
		layers
			.into_iter()
			.nth(min_layer)
			.expect(&format!("Invalid layer index: {}", min_layer))
			.flatten()
			.fold((0, 0), |(ones, twos), x| match x {
				'1' => (ones + 1, twos),
				'2' => (ones    , twos + 1),
				_   => (ones    , twos)
		})
	};

	println!("Corruption check: {}", ones * twos);

	// Part 2

	let layers = layers();
	let buffer: Vec<u8> = layers.into_iter().map(|layer|
		layer.flatten().map(|x| match x {
			'0' => Some(Color::Black as u8),
			'1' => Some(Color::White as u8),
			_   => None
	}))
	.fold(vec![None; WIDTH * HEIGHT], |state, layer| {
		state.iter().zip(layer).map(|(a, b)| a.or(b)).collect()
	})
	.iter().map(|x| x.unwrap_or(Color::Invalid as u8)).collect();

	image::save_buffer(FILENAME, &buffer, WIDTH as u32, HEIGHT as u32, image::Gray(8))
		.expect("Unable to write decoded password");

	println!("Password decoded to: {}", FILENAME);
}

