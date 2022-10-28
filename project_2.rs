fn main() {
	let tosh:f64 = 2.0 * 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 3.0 * 750_000.0;
	let dell:f64 = 3.0 *2_850_000.0;
	let acer:f64 = 250_000.0;
// average
	let total = tosh + mac + hp + dell + acer;
	let n:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	let avg = total/n;
// output
	println!("The total amount is {}", total );
	println!("Average = {}",avg );
}