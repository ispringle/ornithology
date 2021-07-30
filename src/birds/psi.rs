#![warn(missing_docs)]
use masala::curry;
/// The Psi Combinator returns the result of function `a` called with parameters
/// made up of the results of function `b` called first with parameter `c` and
/// then with parameter `d`.
/// ```
/// use ornithology::birds::psi;
///
/// let add_10 = |x| x + 10;
/// let mul = |x, y| x * y;
/// assert_eq!(psi(mul)(add_10)(10)(4), mul(add_10(10), add_10(4)))
/// ```
#[curry]
pub fn psi<T: Clone>(a: fn(T, T) -> T, b: fn(T) -> T, c: T, d: T) -> T {
    a(b(c.clone()), b(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psi() {
        let add_10 = |x| x + 10;
        let mul = |x, y| x * y;
        assert_eq!(psi(mul)(add_10)(10)(4), mul(add_10(10), add_10(4)))
    }
}
