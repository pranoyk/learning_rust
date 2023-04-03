fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Here largest method replaces the largest_i32 and largest_char method by the use of generic type <T>
// we have to provide std::cmp::PartialOrd for type 'T' so that the greater then '>' operator can be used
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
Below we have three implementatio of Point structs
    - First impl tells us that we can use generic impl with <T> to implement any kind of generic methods
    - Second and third impl tells us that we can implement methods with same name for different type to perform diff operation
    - example use case of these is present in main()
 */
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<i32> {
    fn y(&self) -> i32 {
        self.y
    }
}
impl Point<f32> {
    fn y(&self) -> f32 {
        self.y.floor()
    }
}

// PointWithMultipleType tells us that we must pass two diff generic types if the type of the params in struct are diff
// All the above operations can be done with this struct as well
struct PointWithMultipleType<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.1, y: 2.2 };
    println!("value of x in interger_point is {}", integer_point.x());
    println!("value of y in interger_point is {}", integer_point.y());
    println!("value of y in float_point is {}", float_point.y());
    let int_and_float = PointWithMultipleType { x: 1, y: 1.2 };
}
