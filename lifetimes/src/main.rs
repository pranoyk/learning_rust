use std::fmt::Display;

/*
Here we are using generics, traits and lifetimes
 */
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// here 'a defines lifetime of a variable passed into the function
// without 'a the function would return an error because it does not know return str is referenced from x or y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// here the struct holds a reference which means that an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
    level: i32
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        self.level
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    longest_with_an_announcement(string1.as_str(), string2, "any");
    let _i = ImportantExcerpt {
        part: &string1,
        level: 3,
    };
}