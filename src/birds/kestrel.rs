#![warn(missing_docs)]
/// kestrel :: a -> b -> a
/// The Kestrel Combinator, also known as the K combinator, returns the first
/// thing it was given.
/// ```
/// use ornithology::birds::kestrel;
///
/// assert_eq!(kestrel("bird")("cat"), "bird")
/// ```
pub fn kestrel<T: Clone>(a: T) -> impl Fn(T) -> T {
    move |_b: T| -> T { return a.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kestrel() {
        assert_eq!(kestrel(4)(5), 4);
    }
}
