fn main() {
    // Shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    let x = x + 1;
    let x = x + 1;
    println!("The value of x is: {}", x);

    // when using let the data type can be changed.
    // This saves the need to come up with two separate names like 'a_str' and 'a_num'
    let a = "  string  ";
    println!("The value of a is: {}", a);
    let a = a.len();
    println!("The value of a is: {}", a);

    //Mutability
    let mut y = 1;
    println!("The value of y is: {}", y);
    y = 5;
    println!("The value of y is: {}", y);
}
