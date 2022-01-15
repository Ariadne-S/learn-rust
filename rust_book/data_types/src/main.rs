fn main() {
    // Floating point types
    let _f64 = 2.0; // f64
    let _f32: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("{:?}", (sum, difference, product, quotient, floored, remainder));

    // Boolean
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // The Character (char) type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{:?}", (c, z, heart_eyed_cat));

    // COMPOUND TYPES - can group multiple values into one type.
    
    // Tuples
    // general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{:?}", (five_hundred, six_point_four, one));

    // Arrays
    // Unlike a tuple, every element of an array must have the same type. 
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let f3 = [3; 5];
}
