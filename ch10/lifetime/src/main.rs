// Every reference in Rust has a lifetime, the scope in which the reference is
// valid. Usually this is implicit and inferred by the compiler. However, when
// there're multiple possibilities, you must annotate the lifetime with generic
// lifetime parameters (just like when you annotate the type of variables).
// 
// Lifetimes are a type of generics.


fn main() {
    // lifetime prevents dangling reference.
    // The Rust compiler has a borrow checker that compares scopes to 
    // determine whether all borrows are valid.
    {
        let mut r = &6;       // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            // r = &x;        //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
    // r has a lifetime of 'a but refers to memory in the lifetime of 'b.

    // Lifetime annotation in function signature
    // Example1
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is `{}`", result);

    // Example2
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is `{}`", result);
    }

    // Example3
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result); // Don't compile

    // When returning a reference from a function, the lifetime parameter for 
    // the return type needs to match the lifetime parameter for one of the 
    // parameters. If the reference returned does not refer to one of the 
    // parameters, it must refer to a value created within this function, 
    // which would be a dangling reference because the value will go out of 
    // scope at the end of the function.

    // Lifetime annotation in struct definition
    // So far we have defined structs that only contain owned fields. It's
    // possible to define structs that hold reference. This requires a lifetime
    // annotation on every reference field.

    // Lifetie elision rules: Rust compiler will try to analyze your program
    // and infer the lifetime. If your code fits into the pattern that Rust
    // compiler is aware of, you don't have to add lifetime annotations.

    // Lifetimes on functions/method parameters are "input lifetimes", while
    // lifetimes on return values are called "output lifetimes". Here're the
    // three rules:
    // The 1st rule is that each parameter that is a reference gets its own 
    // lifetime parameter. In other words, a function with one parameter gets 
    // one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two 
    // parameters gets two separate lifetime parameters: 
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
    // The 2nd rule is if there is exactly one input lifetime parameter, 
    // that lifetime is assigned to all output lifetime parameters: 
    // fn foo<'a>(x: &'a i32) -> &'a i32.
    // The 3rd rule is if there are multiple input lifetime parameters, but 
    // one of them is &self or &mut self because this is a method, the lifetime 
    // of self is assigned to all output lifetime parameters. This third rule 
    // makes methods much nicer to read and write because fewer symbols are 
    // necessary.

    // Lifetime annotation in method definition
    // Lifetime names for struct fields always need to be declared after the 
    // impl keyword and then used after the struct’s name, because those 
    // lifetimes are part of the struct’s type.

    // The static lifetime
    // The 'static lifetime means the reference can live through the entire
    // lifetime of the program. All string slices have the 'static lifetime.
}


// Lifetime annotation in function signature
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Without knowing whether x or y will be returned, we don't know whether
    // the return value will be a valid reference. The borrow checker doesn't 
    // know the relationship between the lifetime of x, y, and return value.
    
    // Lifetime annotations do NOT change how long any of the references live. 
    // Just as functions can accept any type when the signature specifies a 
    // generic type parameter, functions can accept references with any 
    // lifetime by specifying a generic lifetime parameter. Lifetime annotations 
    // describe the relationships of the lifetimes of multiple references to 
    // each other without affecting the lifetimes.

    // We want to express that all references (x, y, and return value) must
    // have the same lifetime.
    // The function signature now tells Rust that for some lifetime 'a, the 
    // function takes two parameters, both of which are string slices that 
    // live at least as long as lifetime 'a. The function signature also tells 
    // Rust that the string slice returned from the function will live at 
    // least as long as lifetime 'a. In practice, it means that the lifetime 
    // of the reference returned by the `longest` function is the same as the 
    // smaller of the lifetimes of the references passed in. This is the 
    // constraint we want Rust to enforce.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotation in struct definition
struct ImportantExcerpt<'a> {
    // This tells Rust that any ImportantExcerpt shouldn't outlive the the
    // reference it holds as `part`.
    part: &'a str,
}

// Lifetime annotation in method definition
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Generic type paramter, trait bounds, and lifetimes together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}