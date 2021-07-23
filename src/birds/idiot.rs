#![warn(missing_docs)]
/// idiot :: a -> a
/// The Idiot Combinator, also known as the identity combinator, returns what
/// it is given
/// ```
/// use ornithology::birds::idiot;
///
/// assert_eq!(idiot("bird"), "bird")
/// ```
pub fn idiot<T>(a: T) -> T {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiot() {
        assert_eq!(idiot(4), 4);
    }
}
