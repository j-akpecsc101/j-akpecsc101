fn main() {
	let tv:f64 = 210_000.0;
	let dep:f64 = 5.0/100.0;
	let yr1p = tv - dep*tv;
	let yr2p = yr1p - dep*yr1p;
	let yr3p = yr2p - dep*yr2p;
	println!("Value after 3 years = {}", yr3p );
}