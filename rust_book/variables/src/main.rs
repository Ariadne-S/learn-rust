fn main() {
    // Mutation example
    let mut m = 5;
    println!("The value of m is {}", m);
    
    m = 6;
    println!("The value of m is {}", m);
    
    // Inner Shadowing example
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    
    // Type change shadowing example
    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);
}
