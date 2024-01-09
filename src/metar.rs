fn main() {
	let argv: Vec<String> = std::env::args().collect();
	if argv.len() < 2 {
		println!("usage: metar [airport code(icao)]");
		return
	}
	let airport_code = &argv[1];
	let response = reqwest::blocking::get(format!("https://metar.vatsim.net/{}", airport_code).as_str()).unwrap().text().unwrap();
	println!("{}",response);
}