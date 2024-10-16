#![no_std]
#![doc = include_str!("README.md")]

use core::ops::Deref;

/// Split a string into a pair of first character and the rest.
pub fn split_first_char(text: &str) -> Option<(char, &'_ str)> {
    let mut iter = text.chars();
    let first = iter.next()?;
    let rest = iter.as_str();
    Some((first, rest))
}

/// Convenient trait to call [`split_first_char`] as a method.
pub trait SplitFirstChar<'a>: Deref<Target = str> + Sized {
    /// Split a string into a pair of first character and the rest.
    fn split_first_char(self) -> Option<(char, &'a str)>;
}

impl<'a> SplitFirstChar<'a> for &'a str {
    fn split_first_char(self) -> Option<(char, &'a str)> {
        split_first_char(self)
    }
}

#[cfg(test)]
mod test {
    use super::{split_first_char, SplitFirstChar};

    #[test]
    fn test_split_first_char() {
        assert_eq!("abc".split_first_char(), Some(('a', "bc")));
        assert_eq!("x".split_first_char(), Some(('x', "")));
        assert_eq!("".split_first_char(), None);
    }

    #[test]
    fn lifetime() {
        fn _use_function(text: &str) -> Option<&'_ str> {
            let (_, rest) = split_first_char(text)?;
            Some(rest)
        }

        fn _use_method(text: &str) -> Option<&'_ str> {
            let (_, rest) = text.split_first_char()?;
            Some(rest)
        }
    }
}
