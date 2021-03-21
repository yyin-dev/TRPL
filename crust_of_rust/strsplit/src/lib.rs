// Reference: https://youtu.be/rAl-9HwD858

use std::iter::Iterator;

// Use multiple lifetimes when more than more of the field is reference;
// and they should have different lifetimes.
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
    fn next(& mut self) -> Option<Self::Item> {
        // Wrong
        // let ref mut remainder = self.remainder?;
        // Correct
        let remainder = self.remainder.as_mut()?;  // as_mut()
        if let Some(delim_start) = remainder.find(self.delimiter) {
            let found = &remainder[..delim_start];
            *remainder = &remainder[(delim_start + self.delimiter.len())..];
            Some(found)
        } else {
            self.remainder.take() // fn take(&mut self) -> Option<T>
        }
    }
}

fn explain() {
    // Let's discuss why `let ref mut remainder = self.remainder?` is wrong.
    // 1. What is the `ref mut` syntax? 
    // Let's first talk about another syntax, also commonly used in `match` for 
    // pattern matching: '&'.
    let contains_ref: Option<& i32> = Some(&1);
    let i: i32 = match contains_ref {
        // Option<&i32>
        Some(&num) => num,
        None => 0,
    };
    // Patterns are matched exactly. 
    //  Option<& i32> 
    //  Some  <& num>
    // So the type of num is `i32`. This is totally different from `ref`:
    let contains_ref: Option<&i32> = Some(&1);
    let i_ref = &1;
    let i : &&i32 = match contains_ref{
        // Option<&i32>
        Some(ref num) => num,
        None => &i_ref,
    };
    // However, `ref` is not part of the pattern. Think of it as an operator on 
    // the match result.
    //  Option<&i32>
    //  Some  < num>
    // So num is &i32. Then `ref` takes the reference of `num`, so `num` 
    // eventually becomes & &i32.
    // In short, `&` is part of the matching pattern, while `ref` is an operator
    // on the match result. 
    // Similarly, `&mut` is a pattern, while `ref mut` is an operator.
    // 2. Every `let` is a pattern match, so you can use `ref mut` in `let`.
    // The following two lines are equivalent:
    let a: i32 = 1;
    let ref ref1 = a; /* &i32 */
    let ref2 = &a;    /* &i32 */
    // 3. The question mark operator (?)
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
    // "If applied to Some(x), it will unwrap the value to evalute to x".
    // So it's equivalent to
    // 
    // return match(op) {
    //     Some(v) => v,
    //     None => None,
    // }
    // Reference: https://stackoverflow.com/a/42921174/9057530
    // 4. The &str type implements the Copy trait. 
    // Reference: https://stackoverflow.com/a/59995966/9057530
    // 5. When you do:
    //  let ref mut remainder = self.remainder?;
    // The &str inside the Option is copied, but not moved. Thus, you are
    // modifying a copy of the reference inside `self.remainder`, but no
    // the actual reference. So, you get infinite loop.
    // When using `as_mut`, the reference is moved. So it works as wanted.
}

pub fn until_next<'a>(s: &'a str, c: char) -> &'a str {
    StrSplit::new(s, &format!("{}", c)) // This involves memory allocation
        .next()
        .expect("StrSplit should return at least one")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let found: Vec<&str> = StrSplit::new("a b c d", " ").collect();
        assert_eq!(found, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn tail() {
        let found: Vec<&str> = StrSplit::new("a b c d ", " ").collect();
        assert_eq!(found, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn until_next_test() {
        assert_eq!(until_next("hello world", 'o'), "hell");
    }
}
