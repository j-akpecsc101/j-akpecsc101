fn main() {
    let fees = 25_000;
    println!("fees is {}",fees);

    fees = 35_000;
    println!("fees changed to {}",fees );  
}
// this code cant work since fees is an immutable variable
// (its value cant be changed)