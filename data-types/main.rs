fn main() {
    let tup: (i32, bool, char, f64) = (-43, true, 'c', 5.4);

    let (w, x, y, z) = tup;

    println!("The values of w is: {}, x is: {}, y is: {} and z is: {} ", w, x, y, z);
}