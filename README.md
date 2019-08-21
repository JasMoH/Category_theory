# Category_theory
Working through problems posed in Bartosz Milewski's Category Theory for Programmers. Please send help!

I'm reading [this excellent PDF](https://github.com/hmemcpy/milewski-ctfp-pdf)

# Why am I doing this to myself?

I've been wanting to pick up category theory as a formalization of design patterns and foundation of functional programming. I'm sick of C/C++'s garbage, so I've also been meaning to learn Rust. I hear rust recently added GATs, which I hear makes this doable without wanting to cry. May as well pull off the band-aid and do both at once!

# My current progress
I've read through all of section 1 and worked through some of the problems, to get a gist of the topic and a feel for where this is going. I'm now going back and actually working all the problems, taking clearer notes on the definitions, etc.

* [Chapter 1: Category: The Essence of Composition](ch1.md)
* [Chapter 2: Types and Functions](ch2.md)
* [Chapter 3: Categories Great and Small](ch3.md)
* [Chapter 4: Kleisli Categories](ch4.md)

# Other (semi)Related Resources I'm reading, related concepts, and thoughts

This nice series on GATs in rust
* http://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/
* http://smallcultfollowing.com/babysteps/blog/2016/11/03/associated-type-constructors-part-2-family-traits/
* http://smallcultfollowing.com/babysteps/blog/2016/11/04/associated-type-constructors-part-3-what-higher-kinded-types-might-look-like/
* http://smallcultfollowing.com/babysteps/blog/2016/11/09/associated-type-constructors-part-4-unifying-atc-and-hkt/


## Type constructors as functions
This concept is making more sense now that I've seen that the type system in rust is [Turing complete](https://www.reddit.com/r/rust/comments/2o6yp8/brainfck_in_rusts_type_system_aka_type_system_is/). This makes the idea that type constructors "are doing work" more tangible to me.

## The light at the end of the tunnel
having skimmed the first third of the book, I'm starting to get a picture of why one would care about this, other than for its academic value. The discussion of the Yoneda lemma provided clear motivation. I by no means understand this yet, but anything that is universally true in a given field, and provides a clear construction of alternate approaches is usually worth learning about. especially when 'alternate approaches' allow one to shuffle form of computations to avoid the sorts of difficult issues that arise in complicated, real world computing environments.

