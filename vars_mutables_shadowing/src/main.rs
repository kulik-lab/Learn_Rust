fn main() {
    // mutables
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3hrs in seconds is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("y in the inner scope: {y}");
    }
    println!("y in the outer scope: {y}");

    // shadowing II
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // this would raise an error:
    //      let mut spaces = "   ";
    //      spaces = spaces.len();
}

