#![allow(unused)]

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // `Iterator` trait defined in the standrad library.
    // pub trait Iterator {
    //     // `type Item` defines an associated type with the trait. This means
    //     // that `Item` will be the return type from the iterator.
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;

    //     // methods with default implementations elided
    // }
    
    // Rust compiler optimizes iterator very well: zero-cost abstraction.
    // "clearly express high-level ideas at low-level performance"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new() // 1, 2, 3, 4, 5
            .zip(Counter::new().skip(1)) // (1,2), (2,3), (3,4), (4,5)
            .map(|(a, b)| a * b) // 2, 6, 12, 20
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}