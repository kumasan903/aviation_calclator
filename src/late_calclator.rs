fn main() {
	let argv: Vec<String> = std::env::args().collect();
	if argv.len() < 2 {
		println!("usage: dl [ground_speed]");
		return
	}
	let ground_speed:i32 = argv[1].parse().unwrap();
	let descent_late:i32 = ground_speed * 524 / 100;
	println!("descent_late: {descent_late} ft/min")
}