#![warn(missing_docs)]
use masala::curry;
/// The Becard (B3) Combinator applies three functions to an input
/// ```
/// use ornithology::birds::becard;
///
/// let add_10 = |x| x + 10;
/// assert_eq!(becard(add_10)(add_10)(add_10)(10), add_10(add_10(add_10(10))));
/// ```
#[curry]
pub fn becard<T>(a: fn(T) -> T, b: fn(T) -> T, c: fn(T) -> T, d: T) -> T {
    a(b(c(d)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_10(x: isize) -> isize {
        x + 10
    }

    #[test]
    fn test_becard() {
        assert_eq!(
            becard(add_10)(add_10)(add_10)(10),
            add_10(add_10(add_10(10)))
        );
    }
}
