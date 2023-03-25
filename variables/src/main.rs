
fn main() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("x in inner scope is: {x}");
    }
    println!("x in outer scope is: {x}");

    let spaces = "	";
    let spaces = spaces.len();
    println!("spaces = {spaces}");
{
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

	let tup = (500, 6.4, 2);

	let (_x, y, _z) = tup;

	println!("the val of y is: {y}");

	let _array = [1, 2, 3, 4, 6];

	// let a = [3; 5] creates [3, 3, 3, 3, 3]
	

}									
}