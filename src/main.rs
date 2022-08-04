fn main() {
    // * regular variables

    // variables in rust are IMMUTABLE by default
    // ! this will not compile
    // let x = 3;
    // println!("The value of x is: {x}");
    // x = 5;
    // println!("The value of x is: {x}");

    // we can make variables mutable using the 'mut' keyword
    let mut x = 3;
    println!("The value of x is: {x}");
    x = 5;
    println!("The value of x is: {x}");

    // * constant variables
    // like a regular var, but you cannot use the 'mut' keyword, and the type MUST be annotated

    // ! neither of these lines will compile
    // const INCHES_PER_FOOT = 12;
    // const mut INCHES_PER_FOOT: u8 = 12;

    const INCHES_PER_FOOT: u8 = 12;

    println!("The value of INCHES_PER_FOOT is: {INCHES_PER_FOOT}");

    // * shadowing
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z in the outer scope is: {z}");
}
