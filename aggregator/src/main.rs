use aggregator::{Summary, Tweet};

/*
We can also use traits as parameters in function
Another way to write the below code is  - 
    pub fn notify<T: Summary>(item: &T) {
       println!("Breaking news! {}", item.summarize());
    }
This is called trait bound
 */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
Below is an example in case we want to pass two params of traits
Another way to write the below code is - 
    pub fn notify<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news 1! {}", item1.summarize());
        println!("Breaking news 2! {}", item2.summarize());
    }
In case we expect the param to implement both the traits we can use 
    pub fn notify(item: &(impl Summary + Display)) {
        OR
    pub fn notify<T: Summary + Display>(item: &T) {
 */
pub fn notify_twice(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news 1! {}", item1.summarize());
    println!("Breaking news 2! {}", item2.summarize());
}

/*
In case both out params in the functions accept diff traits. We can write the code -
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

But in the above example we may end up with a lot of clutter and unreadable code.
To solve that we can use the 'where' clause provided
        fn some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            unimplemented!()
        }

Traits can also be bound to struct types as implemented in generic_data_types
 */

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };



    println!("1 new tweet: {}", tweet.summarize());
}