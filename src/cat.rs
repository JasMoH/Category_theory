// chapter 1
// problem 1: identity function in rust

pub fn id<T>(x: T) -> T{
	return x
}

/**
problem 2: general function composition
problem 3: test that compose respects identity
```
let f1 = cat::compose(|val| val + 1, &cat::id);
assert_eq!(5,f1(4));
```

```
let f2 = cat::compose(&cat::id, |val| val + 1);
assert_eq!(5,f2(4));
```

```
let f3 = cat::compose(&cat::id, &cat::id);
assert_eq!(-55,f3(-55));
```

```
pub fn returnFive<T>(x: T) -> i32{
	return 5;
}

let f4 = cat::compose(returnFive,&cat::id );
assert_eq!(5,f4(()));
```
*/

//I'm assuming that functions have been curried to avoid figuring out how to make function signatures more generic for the moment. 
pub fn compose<T1,T2,T3>(f: impl Fn(T1) -> T2, g: impl Fn(T2) -> T3) 
		-> impl Fn(T1) -> T3{
			move |x| (g(f(x)))
}


use std::collections::HashMap;

/**
```
let f = |v|  v +1;
let mut mem = cat::Memoize::new(f);

assert_eq!(6,mem.eval(5));
assert_eq!(6,mem.eval(5));
```

```
extern crate rand;

use rand::Rng;

fn f<T>(x: T){
	let mut rng = rand::thread_rng();
	rng.gen()
}

let mut mem = cat::Memoize::new(f);

let one_true_random_value = mem.eval(());

assert_eq!(one_true_random_value,mem.eval(()));
assert_eq!(one_true_random_value,mem.eval(()));
assert_eq!(one_true_random_value,mem.eval(()));
```
*/

pub struct Memoize<T1, T2>
	where
	T1: Eq + std::hash::Hash,
	T2: std::marker::Sized
{
	cache: HashMap<T1,T2>,
	f: fn(T1) -> T2
}


//chapter 2
//implementing memoizing without using the cashed crate
impl<'a, T1, T2> Memoize<T1,T2> 
	where
	T1: 'a + Eq + std::hash::Hash + Clone,
	T2: std::marker::Sized + Clone
{
	pub fn new(func: fn(T1) -> T2) -> Memoize<T1,T2>{
		Memoize{
			cache: HashMap::new(), 
			f: func
		}
	}

	pub fn eval(&mut self, x: T1) -> T2{
		match self.cache.get(&x).map(|x| x.clone()){
			Some(result) => result,
			None => {
				let res = (self.f)(x.clone());
				self.cache.insert(x,res.clone());
				return res.clone();
			}
		}
	}
}


