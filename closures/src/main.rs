fn main() {
    /*
    In the below example we can demonstrate how functions and closures are similar in syntax.
    We also demonstrate that closures require very minimal code without the type annotations or the curly brackets.
     */
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!("{}", add_one_v1(1));
    println!("{}", add_one_v2(1));
    println!("{}", add_one_v3(1));
    println!("{}", add_one_v4(1));
}
