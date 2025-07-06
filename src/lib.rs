//! # yerevan.rs
//! yerevan.rs is a PoC package where I experiment with adding functionality and syntax of Computation Expressions (CEs)
//! from F#, aka do-notations from Haskell, but with several changes and improvements.
//!
//! Main component of yerevan.rs package is the [`yer!`] macro.
//!
//! ## Note
//! The things called [computation expressions](https://fsharpforfunandprofit.com/posts/computation-expressions-intro/) from F# are almost the same as [do-notations](https://en.wikibooks.org/wiki/Haskell/do_notation) from Haskell.
//!
//! ## Features and syntax of [`yer!`] macro:
//! **let! unwrapping-binding** <br>
//! Syntax: `let! your_var = some_expression_to_bind;` <br>
//! F# way: `let! your_var = some_expression_to_bind` <br>
//!
//! **let usual var-defining** <br>
//! Syntax: `let your_var = some_expression;` <br>
//! F# way: `let your_var = some_expression` <br>
//!
//! **do! unwrapping expression without binding** <br>
//! Syntax: `do! some_expression_to_unwrap;` <br>
//! F# way: `do! some_expression_to_unwrap` <br>
//!
//! **do just executing expression** <br>
//! Syntax: `let some_expression;` <br>
//! F# way: `do some_expression` <br>
//!
//! **ret wrapping and returning** <br>
//! Syntax: `ret expression_to_return;` <br>
//! F# way: `return expression_to_return` <br>
//!
//! **ret! return without wrapping** <br>
//! Syntax: `ret! expression_to_return;` <br>
//! F# way: `return! expression_to_return` <br>
//!
//! **yield wrapping and yielding** <br>
//! Syntax: `yield expression_to_yield;` <br>
//! F# way: `yield expression_to_yield` <br>
//!
//! **yield! flattening value of accamulated type (flattened yielding)** <br>
//! Syntax: `yield! expression_to_yield_from;` <br>
//! F# way: `yield! expression_to_yield_from` <br>
//!
//! **`StructName =>` setting up or changing the CE-struct (struct which is providing specific methods for yer! macro)** <br>
//! Syntax: `YourStructName => ...` <br>
//! F# way: `yourStructInstance { ... }` <br>
//!
//! **`run` takes last returned value and puts it into `YourStruct::run` method** <br>
//! Syntax:
//! ```text
//! run StructName => ...;
//! ret state_for_run
//! 
//! // or
//!
//! `StructName >> ...;
//! ret state_for_run
//! ```
//! F# way: `yourStructInstance { ... }` <br>
//!
//! **if-else generating Rust's if-statement**
//! Syntax:
//! ```text
//! if (bool_expr) { yield "something"; }
//! else if (bool_expr) { yield "something else"; }
//! else { yield "nothing"; }
//!
//! // or
//!
//! if (bool_expr) { yield "something"; }
//! else zero; // needs `zero` method
//! ```
//! F# way: `if true then yield "something"
//
//! ## How to implement these methods in your struct?
//! ### Note to Methods API
//! There is no specific type you have to use by using methods signatures defined below.
//!
//! The [yer!] macro is very flexible, so you must only use your types in a right way knowing
//! how CEs work inside.
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
//! - `yield!`
//! ```ignore
//! pub fn combine<T, U>(val: T, state: U) -> U;
//! pub fn ret_yield_from<T>(val: T) -> T;
//! ```
//! F# way:
//! ```fsharp
//! member _.Combine<'T, 'U>(value: 'T, state: 'U) : 'U
//! member _.YieldFrom<'T>(value: CEType<'T>) : CEType<'T>
//! ```
//! - `run`
//! ```ignore
//! pub fn run<T, U>(state: T) -> U;
//! ```
//! F# way:
//! ```fsharp
//! member _.Run<'T, 'U> (state: 'T) : 'U
//! ```
//!
//! ## Why CEs in Rust?
//! A section for those who don't know why CEs are so useful.
//!
//! In Rust you can use the ? operator which is actually just a syntax sugar for situations, where you need to safely unwrap some value.
//! The ? operator is based on the type you return from the function which makes it useless in cases when you need to unwrap a different type. And some packages create their own implementations of Result or Option types which also makes this operator less useful.
//!
//! Also in Rust you can define your own macros to extend the language possibilities and use it just like CEs but with your own syntax.
//! But creating a big macros can be tricky and difficult. And if you created your own syntax it doesn't mean that this syntax will be readable for you or anyone else.
//!
//! CEs are just functions you call in syntax-sugared way. So you can create your own custom control-flow, just like with macros but with standard syntax.
//! For example, in CE-functions you can implement: builder pattern, safe unwrapper of enums (like Option or Result), etc.

#[macro_use]
pub mod yer_macro;
pub mod railway_exec;
