//! # yerevan.rs
//! yerevan.rs is a package providing functionality and syntax of computation expressions (CEs)
//! from F#, but with several changes and improvements.
//!
//! Main component of yerevan.rs package is the [`yer!`] macro.
//!
//! ## Note
//! The things called [computation expressions](https://fsharpforfunandprofit.com/posts/computation-expressions-intro/) from F# are the same as [do-notations](https://en.wikibooks.org/wiki/Haskell/do_notation) from Haskell.
//!
//! ## Features and syntax of [`yer!`] macro:
//! |Feature|Syntax|F# way|
//! |-|-|-|
//! |`let!` unwrapping-binding|`let! your_var = some_expression_to_bind;`|`let! your_var = some_expression_to_bind`|
//! |`let` usual var-defining|`let your_var = some_expression;`|`let your_var = some_expression`|
//! |`do!` unwrapping expression without binding|`do! some_expression_to_unwrap;`|`do! some_expression_to_unwrap`|
//! |`do` just executing expression|`let some_expression;`|`do some_expression`|
//! |`ret` wrapping and returning|`ret expression_to_return;`|`return expression_to_return`|
//! |`ret!` return without wrapping|`ret! expression_to_return;`|`return! expression_to_return`|
//! |`yield` wrapping and yielding|`yield expression_to_yield;`|`yield expression_to_yield`|
//! |`run` takes last returned value
//! |`StructName =>` setting up or changing the CE-struct (struct which is providing specific methods for yer! macro)|`YourStructName => ...`|`let yourStruct = YourStructName()`<br>`yourStruct { ... }`|
//!
//! ## How to implement these methods in your struct?
//! ### Note to Methods API
//! There is no specific types you have to use by using methods signatures defined below.
//!
//! The [yer!] macro is very flexible, so you must only use your types in a right way knowing
//! how CEs are work inside.
//!
//! Also check the examples in [yer!]-page and tests in [tests/common.rs file in the repo](https://github.com/marat0n/yerevan.rs/blob/dev/tests/common.rs) to better understand how CEs work inside.
//! ### Methods API
//! - `let!`, `do!`
//! ```ignore
//! pub fn bind<T, U>(val: CEStruct<T>, f: &dyn Fn(T) -> CEStruct<U>) -> CEStruct<U>;
//! ```
//! F# way:
//! ```fsharp
//! member _.Bind<'T, 'U>(value: CEType<'T>, f: 'T -> CEType<'U>) : CEType<'U>
//! ```
//! - `ret`
//! ```ignore
//! pub fn ret<T>(val: T) -> CEStruct<T>;
//! ```
//! F# way:
//! ```fsharp
//! member _.Return<'T>(value: 'T) : CEType<'T>
//! ```
//! - `ret!`
//! ```ignore
//! pub fn ret_from<T, U>(val: T) -> U;
//! ```
//! F# way:
//! ```fsharp
//! member _.ReturnFrom<'T>(value: 'T) : <'T>
//! ```
//! - `yield`
//! ```ignore
//! pub fn combine<T, U>(val: T, state: U) -> U;
//! pub fn ret_yield<T, U>(val: T) -> U;
//! ```
//! F# way:
//! ```fsharp
//! member _.Combine<'T, 'U>(value: 'T, state: 'U) : 'U
//! member _.Yield<'T>(value: 'T) : CEType<'T>
//! ```
//!
//! ## Why CEs in Rust?
//! Section for those who don't know why CEs are so useful.
//!
//! In Rust you can use the `?` operator which is actually just a syntax sugar for situations,
//! where you need to unwrap some value, but do it in a safe way.
//! The `?` operator is based on the type you
//! return from the function which is makes it useless in cases when you need to unwrap
//! a different type. And some packages create their own implementations of Result or Option
//! types which is also makes this operator less useful.
//!
//! CEs are just functions you call in syntax-sugared way. And they are more flexible to types you
//! use inside the CE you write. In CEs you can also make your own implementation of the unwrapping (it can be
//! useful for logging in case of Options or Results). And this is not an end to CEs possibilities,
//! they can be used as a Builder-pattern and much more.

#[macro_use]
pub mod yer_macro;
