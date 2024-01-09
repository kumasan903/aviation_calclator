fn main() {
	let argv: Vec<String> = std::env::args().collect();
	if argv.len() < 2 {
		println!("usage: a [qnh(hpa)]");
		return
	}
	let qnh = argv[1].parse::<f32>().unwrap();
	let altimeter = qnh / 0.33864;
	println!("{:.0}",altimeter);
}