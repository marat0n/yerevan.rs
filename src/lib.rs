//! # yerevan.rs
//! yerevan.rs — the package providing functionality and syntax of CEs (computation expressions)
//! from F# but with several changes and improvements.
//!
//! Main component of yerevan.rs package is the [`yer!`] macro.
//!
//! ## Note
//! The thing called [computation expressions](https://fsharpforfunandprofit.com/posts/computation-expressions-intro/) from F# is the same as [do-notations](https://en.wikibooks.org/wiki/Haskell/do_notation) from Haskell.
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
//! |`StructName =>` setting up or changing the CE-struct (struct which is providing specific methods for yer! macro)|`YourStructName => ...`|`let yourStruct = YourStructName()`<br>`yourStruct { ... }`|
//!
//! ## How to implement these methods for my struct?
//! ### Note to Methods API
//! There is no such thing as specific or concrete types you must use by using methods signatures
//! defined below.
//!
//! The [yer!] macros is very flexible, so you must only to use your types in a right way knowing
//! how CEs are working inside.
//!
//! Also try to check examples in [yer!]-page and tests in [tests/common.rs file in repo](https://github.com/marat0n/yerevan.rs/blob/dev/tests/common.rs) to better understand how CEs are working inside.
//! ### Methods API
//! - `let!`
//! ```ignore
//! pub fn bind<T, U>(val: CEStruct<T>, f: &dyn Fn(T) -> CEStruct<U>) -> CEStruct<U>;
//! ```
//! F# way
//! ```fsharp
//! member _.Bind<'T, 'U>(value: CEType<'T>, f: 'T -> CEType<'U>) : CEType<'U>
//! ```
//! - `ret`
//! ```ignore
//! pub fn ret<T>(val: T) -> CEStruct<T>;
//! ```
//! F# way
//! ```fsharp
//! member _.Return<'T>(value: 'T) : CEType<'T>
//! ```
//! - `ret!`
//! ```ignore
//! pub fn ret_from<T, U>(val: T) -> U;
//! ```
//! F# way
//! ```fsharp
//! member _.ReturnFrom<'T>(value: 'T) : <'T>
//! ```
//! - `yield`
//! ```ignore
//! pub fn combine<T, U>(val: T, state: U) -> U;
//! pub fn ret_yield<T, U>(val: T) -> U;
//! ```
//! F# way
//! ```fsharp
//! member _.Combine<'T, 'U>(value: 'T, state: 'U) : 'U
//! member _.Yield<'T>(value: 'T) : CEType<'T>
//! ```
//!
//! ## Why CEs in Rust?
//! Section for those who doesn't know why are CEs are very useful.
//!
//! In Rust you can use the `?` operator which actually just a syntax sugar for situations when you
//! need to unwrap some value but do it in a safe way. The `?` operator is based on the type you
//! returning from the function which is makes it not so useful in cases when you need to unwrap
//! a different type. And some packages are creating their own implementation of Result or Option
//! types which is also makes this operator less useful.
//!
//! But CEs are just functions you call in syntax-sugared way. And it is more flexible to types you
//! use inside. In CEs you also can make your own implementation of that unwrappings (it can be
//! useful for logging in case of Options or Results). And it is not the end of CEs possibilities,
//! they are can be used as a Builder-pattern and much more.

#[macro_use]
pub mod yer_macro;
