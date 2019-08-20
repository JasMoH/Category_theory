# 1

In rust:
```rust
fn id<T>(x: T) -> T{
	return x
}
```

# 2
```rust

pub fn compose<T1,T2,T3>(f: impl Fn(T1) -> T2, g: impl Fn(T2) -> T3) 
		-> impl Fn(T1) -> T3{
			move |x| (g(f(x)))
}

```

# 3
```rust
let f1 = chapter1::compose(|val| val + 1, &chapter1::id);
assert_eq!(5,f1(4));
```
```rust
let f2 = chapter1::compose(&chapter1::id, |val| val + 1);
assert_eq!(5,f2(4));
```
```rust
let f3 = chapter1::compose(&chapter1::id, &chapter1::id);
assert_eq!(-55,f3(-55));
```

# 4
Most webpages don't have links to themselves, so identity morphisms must also be added. However, many webpages change when reloaded, including their links (morphisms), so the category will not be constant in time, network partitions, and the like. 

A naive / statically mirrored world wide web can be a category where links are morphisms:
* up to the problems posed by distributed systems (CAP theory and the like)
* provided that compositions of morphisms (i.e. following 2 links sequentially) are procedurally added to the category
* and identity morphisms (reloading) are added

# 5
People are not strictly speaking 'friends' of themselves in Facebook, so the same identity issue for the world wide web is present for Facebook. additionally, the same time variant nature of the world wide web is present here.

I would argue that 'strict' friendship is not compose-able, as a friend of my friend is not necessarily my friend. A 'week' definition of friendship counting 'friend of friend' is compose-able. therefore, facebook is a category with people as objects and friendship as morphisms if friend of friend composition is permitted.

# 6
Any directed graph can construct a category provided:
1) identity morphisms (edges) are added to all nodes
2) all compositions of edges are added to the category as new composition of morphism edges.

![Addition of identity morphisms and compositions to a DG to create a category](chapter_1/ch1%236.png)