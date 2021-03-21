// Reference: https://youtu.be/rAl-9HwD858
// This is an improved version of StrSplit in `strsplit` trait.

use std::iter::Iterator;

// Using this Delimiter trait, StrSplit doesn't have to store two references
// as fields (which are supposed to have different lifetimes), and thus
// we don't have to use multiple references.
// This also eliminates the memory allocation for char as delimiter.
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // Wrong
        // let ref mut remainder = self.remainder?;
        // Correct
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let found = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(found)
        } else {
            self.remainder.take() // fn take(&mut self) -> Option<T>
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + 1))
    }
}

pub fn until_next<'a>(s: &'a str, c: char) -> &'a str {
    StrSplit::new(s, c)
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
