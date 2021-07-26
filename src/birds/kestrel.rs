#![warn(missing_docs)]
use masala::curry;

/// kestrel :: a -> b -> a
/// The Kestrel Combinator, also known as the K combinator, returns the first
/// thing it was given.
/// ```
/// use ornithology::birds::kestrel;
///
/// assert_eq!(kestrel("bird")("cat"), "bird")
/// ```
#[curry]
pub fn kestrel<T: Clone>(a: T, _b: T) -> T {
    a.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kestrel() {
        assert_eq!(kestrel(4)(5), 4);
    }
}
