fn main()
{
	let argv: Vec<String> = std::env::args().collect();
	if argv.len() < 3 {
		println!("usage: dc [current_altitude] [target_altitude]");
		return
	}
	let current_altitude:f32 = argv[1].parse().unwrap();
	let target_altitude:f32 = argv[2].parse().unwrap();
	let required_distance = (current_altitude - target_altitude) / 3.18;
	println!("required_distance: {:.1}nm", required_distance);
}