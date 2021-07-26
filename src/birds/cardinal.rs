#![warn(missing_docs)]
use masala::curry;
use std::ops::Div;

/// cardinal :: a -> b -> c -> b -> a -> c
/// The Cardinal Combinator, also known as the C combinator, takes a function
/// and two values, and then flips the two values and runs them in the function.
/// ```
/// use ornithology::birds::cardinal;
/// let divide = |x, y| x / y;
/// assert_eq!(cardinal(divide)(1)(3), divide(3, 1))
/// ```
#[curry]
pub fn cardinal<T: Clone>(a: fn(T, T) -> T, b: T, c: T) -> T {
    a(c.clone(), b.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn divide<T: Div<Output = T>>(a: T, b: T) -> T {
        a / b
    }

    #[test]
    fn test_cardinal() {
        assert_eq!(cardinal(divide)(4)(5), divide(5, 4));
    }
}
