# Rust Book

## What could be better?

**Tell me the notable features of Rust earlier!**

* Spends too much time telling about the basics (_chapter 1-6_). However, I believe the real audience of Rust is not novices, but veterens.

* Also, the book spends a lot of time explaining the module, crate, package structure (_chapter 7_) way before they are really needed. I don't care about the facade (re-exporting).

* I remember wondering about what are traits from the beginning.

## The most useful chapter was these ones:
* Programming a Guessing Game
* Understanding Ownership

## What should have included from the start?
* A tour of stdlib: io, env, net, idk?!
* Rust editions could be clarified. IDK what are they.

---

## Chapter 1: Getting Started

* `rustup` installation OK.
* Don't talk about `rustc`, just tell me `cargo [new|run]`.
* Unnecessarily talking about `Cargo.toml`. Tell me you're going to explain me later. And you do in the Guessing game chapter.

## Chapter 2: Programming a Guessing Game

* Awesome chapter, nothing to say.

## Chapter 3: Common Programming Concepts

* The worst chapter ever.
* Explains the basic concepts very DRY. As if it's a spec of the language...

## Chapter 4: Understanding Ownership

* Awesome chapter.
* Talks about stack/heap difference way earlier. Not needed at this stage. Talk about it when talking about copying integers (_where there is no need clone them_) etc.
* Why explain slices here? Explain where it is needed, later on.

## Chapter 5: Structs

* Another boring chapter. Dry like the 2nd one.
* Methods and the Rectangle example at the end were enough.

## Chapter 6: Enums and Pattern Matching

* Dry... Boring... Although there are interesting features like `match`, data binding to `enum`s, etc. These aspects should have been highlighted more.

## Chapter 7: Managing Growing Projects with Packages, ...

* WTF?! Why is this here? I didn't create a big project yet. Why should I want to manage my "growing" project? Not yet. Explain me later on.

* Why do you talk about the glob operator if it's often used when testing!

## Chapter 8: Common Collections

* "deref coercion": Too much unnecessary complexity at this point (which is a topic of Chapter 15!). So, just show me what can be done and not without explaining the inner details yet. I can look them up if I need to.

