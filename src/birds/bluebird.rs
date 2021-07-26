type BluebirdT0<T> = impl Fn(T) -> T;
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
pub fn bluebird<T>(b: fn(T) -> T) -> impl Fn(fn(T) -> T) -> BluebirdT0<T> {
    move |c| move |a| b(c(a))
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
