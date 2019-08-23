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


//chapter 4

/**
```
assert!(cat::safe_reciprocal(0.0).is_none());
assert_eq!(1.0,cat::safe_reciprocal(1.0).unwrap());
```
*/

pub fn safe_reciprocal(x: f64) -> Option<f64>{
	if x == 0.0{
		return None
	}
	else{
		return Some(1.0/x)
	}
}

pub fn safe_root(x: f64) -> Option<f64>{
	if x > 0.0 {
		return Some(x.sqrt())
	}
	else{
		return None
	}
}

pub fn safe_id<T>(x: T) -> Option<T>{
	return Some(x)
}

/**
```
let f = cat::safe_compose(cat::safe_reciprocal,cat::safe_root);
assert!(f(0.0).is_none());
assert!(f(-1.0).is_none());
assert_eq!(1.0,f(1.0).unwrap());
assert_eq!(0.5,f(4.0).unwrap());
```
*/
pub fn safe_compose<A,B,C>(f: impl Fn(A) -> Option<B>, g: impl Fn(B)->Option<C>) -> impl Fn(A) -> Option<C>
{
	move |x| {
		let y = f(x);
		match y{
			None => None,
			Some(result) => g(result),
		}
	}
}



//chapter 5

/**
problem 4
implement either


*/

#[derive(Copy, Clone)]
pub struct Either<L,R> {
	left: Option<L>,
	right: Option<R>
}

impl<L,R> Either<L,R>
	{
	pub fn left(val: L) -> Either<L,R>{
		Either{
			left: Some(val),
			right: None
		}
	}

	pub fn right(val: R) -> Either<L,R>{
		Either{
			left: None,
			right: Some(val)
		}
	}

	pub fn is_left(&self) -> bool{
		self.left.is_some()
	}
	pub fn is_right(&self) -> bool{
		return self.right.is_some()
	}

	pub fn get_left(self) -> L{
		return self.left.unwrap()
	}

	pub fn get_right(self) -> R{
		return self.right.unwrap()
	}
}

/**
exercise 5

```

```
*/
pub fn m(e: Either<i64,bool>) -> i64{
	if e.is_left(){
		return e.get_left()
	}
	else{
		if e.get_right(){
			return 0
		}
		else {
			return 1
		}
	}
}