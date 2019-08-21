#1
```rust
pub fn safe_id<T>(x: T) -> Option<T>{
	return Some(x)
}
```

see safe_compose bellow for the lifted composition definition


#2
```rust
pub fn safe_reciprocal(x: f64) -> Option<f64>{
	if x == 0.0{
		return None
	}
	else{
		return Some(1.0/x)
	}
}
```
#3
Definition of safe_root in rust, as well as the lifted identity
```rust
pub fn safe_root(x: f64) -> Option<f64>{
	if x > 0.0 {
		return Some(x.sqrt())
	}
	else{
		return None
	}
}


```


safe_compose is the functor lifting composition to the option category

```rust
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
```

passes the following unit tests
```rust
let f = cat::safe_compose(cat::safe_reciprocal,cat::safe_root);
assert!(f(0.0).is_none());
assert!(f(-1.0).is_none());
assert_eq!(1.0,f(1.0).unwrap());
assert_eq!(0.5,f(4.0).unwrap());
```