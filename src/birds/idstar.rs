#![warn(missing_docs)]
use masala::curry;
/// idstar :: (a, b) -> a -> b
/// The I-star Combinator, also known as the applicator combinator, returns the
/// result of function `a` called with parameter `b`
/// ```
/// use ornithology::birds::idstar;
///
/// let add_10 = |x| x + 10;
/// assert_eq!(idstar(add_10)(12), add_10(12))
/// ```
#[curry]
pub fn idstar<T>(a: fn(T) -> T, b: T) -> T {
    a(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idstar() {
        let add_10 = |x| x + 10;
        assert_eq!(idstar(add_10)(12), add_10(12))
    }
}
