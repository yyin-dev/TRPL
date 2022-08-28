// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// Unlike functions, closures can capture values from the scope in which they’re defined.

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, V>
where
    T: Hash + Eq + Clone,
    U: Clone,
    V: Fn(T) -> U,
{
    // All closures implement at least one of the traits: Fn, FnMut, FnOnce.
    calculation: V,
    values: HashMap<T, U>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Hash + Eq + Clone,
    U: Clone,
    V: Fn(T) -> U,
{
    fn new(calculation: V) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, args: T) -> U {
        match self.values.get(&args) {
            Some(val) => val.clone(),
            None => {
                let key = (&args).clone();
                let res = (self.calculation)(args);
                let v = res.clone();
                self.values.insert(key, v); // fn insert(&mut self, k: K, v: V) -> Option<V>
                res
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Closure doesn't require type annotations for parameters and return
    // values. Functions require type annotations because they are part of an
    // explicit interface. However, closures aren't used in an exposed
    // interface. They are stored in variables and without being exposed.
    // Thus, the compiler is able to infer the types of parameterse and return
    // value.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Closures can capture environment. This is implemented by storing the 
    // values for use in the closure body. This is extra overhead.
    // Closures can capture values from the environment in three ways: taking
    // ownership (FnOnce), borrowing mutably (FnMut), borrowing immutably (Fn).
    //
    // When you create a closre, Rust infers which trait to use based on how 
    // the closure uses the values from the environment. 
    // If you want to force the closre to take ownership of the values it uses
    // in the environment, you can use `move` keyword before the parameter list.
    // 
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
