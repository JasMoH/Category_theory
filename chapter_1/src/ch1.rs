// problem 1: identity function in rust

pub fn id<T>(x: T) -> T{
	return x
}

/// problem 2: general function composition
/// problem 3: test that compose respects identity
///
/// ```
/// let f1 = chapter1::compose(|val| val + 1, &chapter1::id);
/// assert_eq!(5,f1(4));
/// ```
/// ```
/// let f2 = chapter1::compose(&chapter1::id, |val| val + 1);
/// assert_eq!(5,f2(4));
/// ```
/// ```
/// let f3 = chapter1::compose(&chapter1::id, &chapter1::id);
/// assert_eq!(-55,f3(-55));
/// ```

//I'm assuming that functions have been curried to avoid figuring out how to make function signatures more generic for the moment. 
pub fn compose<T1,T2,T3>(f: impl Fn(T1) -> T2, g: impl Fn(T2) -> T3) 
		-> impl Fn(T1) -> T3{
			move |x| (g(f(x)))
}