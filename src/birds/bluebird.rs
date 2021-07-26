use masala::curry;

type AFunc<T> = fn(T) -> T;

/// bluebird :: b -> c -> a -> b -> a-> c
/// The Bluebird Combinator, also known as the B combinator or function
/// composition. This is a little harder to understand, but, essentially,
/// it takes two functions and a value. Then applies the two functions to that
/// value. A B combinator is the same as `b(c(a))`.
/// ```
/// use ornithology::birds::bluebird;
/// let b = |x| x * 2;
/// let c = |x| x - 1;
/// assert_eq!(bluebird(b)(c)(3), b(c(3)))
/// ```
#[curry]
pub fn bluebird<T>(a: AFunc<T>, b: AFunc<T>, c: T) -> T {
    a(b(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluebird() {
        let b = |x| x * 2;
        let c = |x| x - 1;
        assert_eq!(bluebird(b)(c)(3), b(c(3)));
    }
}
