# 1
Show the isomorphism between Maybe a and Either () a

```rust
fn to_either<a>(m: Option<a>) -> Either<a>{
	match m{
		None => Either(()),
		Some(val) => Either(val)
	}
}

fn to_maybe<a>(e: Either<a>) -> Option<a>{
	if e.is_left(){
		None
	}
	else {
		Some(e.get_right())
	}
}

```

# 2 - 4
I'm going to be a little lazy for the moment because I'm a bad student. I can see how adding new derived types / child classes to the shape interface will require changes to the interface to implement circumference, and that will spill out to all children. 

# 5
Show that 𝑎 + 𝑎 = 2 × 𝑎 holds for types (up to isomorphism). Remember that 2 corresponds to Bool, according to our translation
table.

using the translation table for algebraic types
the left of this equation is:
`a + a => Either<a,a>  = Left a | Right a`

and the right:
`2 x a => (bool,a)`

defining an isomorphisms between these two types:

```rust
fn to_pair<a>(e: Either<a>) -> (bool,a){
	if e.is_left(){
		(true,e.get_left())
	}
	else{
		(false,e.get_right())
	}
}

fn to_either<a>(p: (bool,a)) -> Either<a,a>{
	if p[0]{
		Either::left(p.0)
	}
	else {
		Either::right(p.1)
	}
}

```

noting that we car swap the true/false to left/right correspondence without losing any information.
