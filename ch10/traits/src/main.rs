#![allow(unused)]

pub trait Summary {
    fn summarize(&self) -> String;
    fn in_english(&self) -> bool {
        // default trait implementation
        true
    }
    // Default implementations can call other methods in the same trait, even
    // if those other methods don't have a default implementation. This is
    // safe because the compiler will ensure that methods in the trait are
    // implemented.

    // It's impossible to call the default implementation from an overiding
    // implementation.
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    fn new() -> NewsArticle {
        NewsArticle {
            headline: String::from("hl"),
            location: String::from("loc"),
            author: String::from("athr"),
            content: String::from("ctnt"),
        }
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn in_english(&self) -> bool {
        false
    }
}

// Trait as parameter
pub fn notify(can_summarize: &impl Summary) {
    println!("Notify: {}", can_summarize.summarize());
}

// The above `impl Summary` is syntactic sugar for a longer form, called
// "a trait bound", loke the following:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notifyTwoSameTrait(item1: &impl Summary, item2: &impl Summary) {
    // item1 and item2 might be differnt types
}

pub fn notifyTwoSameType<T: Summary>(item1: &T, itme2: &T) {
    // item1 and item2 must be the same type
}

use std::fmt::Display;
pub fn sumarize_and_display(item: &(impl Summary + Display)) {
    // The same as `pub fn summarize_and_display<T: Summary + Display> (item: &T) { ... }`
}

// where clause to improve readability
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    1
}


// Using trait bound to conditionally implement methods
// Conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are 
// called blanket implementations, which is used extensively in Rust standard
// library.
// To implement a toString trait on any type that implements Display:
trait toStringTrait { }

impl<T: Display> toStringTrait for T {
    // ...
}

// Trait as return type
fn return_summarizable() -> impl Summary {
    NewsArticle::new()

    // You can only use `impl Trait` if you are returning a single type. For
    // example, you cannot return `NewsArticle` in `if` and `Tweet` in `else`, 
    // for a function returning `impl Summary`.
    // This is due to way Rust compiler is implemented.
}

fn main() {
    // Traits specify he functionality a particular type can share with other
    // types. It's like `interface` in Golang and Java.

    // One restriction with trait implementations is that we can implement a
    // trait on a type only if either the trait or the type is local to our crate.

    let article = NewsArticle {
        headline: String::from("hl"),
        location: String::from("loc"),
        author: String::from("athr"),
        content: String::from("ctnt"),
    };
    println!("{}, {}", article.summarize(), article.in_english());

    let tweet = Tweet {
        username: String::from("usr"),
        content: String::from("ctnt"),
        reply: true,
        retweet: false,
    };
    println!("{}, {}", tweet.summarize(), tweet.in_english());

    notify(&article);
    notify(&tweet);
}
