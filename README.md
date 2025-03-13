```

                       __| __|
                      |  \__ \
                   _)_|  ____/       //~\
                                   //  ~~\\
              _                 /───   ~~  \\
             / \              ───      ~~  ~~\
            / ~~\\          /           ~~~   \
          //   ~~\\\ ~\   /            ~~~   ~~\\
        //     ~~~ \~─~──── ~~         ~~~~~
   ~──//         ~~~
───────╮  ~~~~   ~     ~ ~ ~~~~~~~ ~~~~~~~~~~~~~
 #  #  │╮    ~~~~~~~~~~~~│  '│╭──── ╭─  ╭^^^^^^^^^
       │╯         │`  │  ╰╮───╯  ╭──╯   │ @ @ @ @
 #  #╭╮│╮            ╭╯   │  ╭───╯  ╭───┼─────────
   ╭─╰╯──^^╮    │`   │.   │──╯      │
 ╭─╰───────╯^╮───────│    │  '│   ╭─│ #  #  #  #
────────╮@ @ │──╮    │  ..│       ╰─│
        │    │  ╰───╭╯   ╭╯         │
 # # # #│@ @ │      │    ╰─╮ '│   ╭─│ #  #  #  #
        │────╯ '│`  │  .   ╰╮ │   ╰─│
 # # # #│       │   │       │       │
        │          ╭╯    .  ╰╮    ╭─│ #  #  #  #
 # # # #│  '│`  ╭──╯  .      ╰╮   ╰─│
        │   │   │     .       │     │
```


# Overview

[yerevan.rs](https://github.com/marat0n/yerevan.rs) is a Proof Of Concept (PoC) Rust library of Computation Expressions (CEs), aka "do-notations" in Haskel,
inspired by F#'s CEs, but with changes and improvements in syntax-design.

> 1. This package is a PoC of the idea that CEs could be very useful in Rust.
> 2. The goal of this PoC is to evolve to a CEs feature proposal in Rust language, if it succeeds.

## Why is this package only a PoC and not a solid solution?
> 1. This package's implementation of CEs uses Rust macros, which slow the Rust compiler down. Besides, CEs are meant to be used often, which will significantly increase compilation time.
> 2. Some syntax features of classical do-notations and CEs are not possible with Rust's `macro_rules`.

# Docs

1. Firstly define the struct which is implementing all the neccessary functions for <span>yerevan.rs</span>\'s computation expressions.
Like this:
```rust
struct SimpleBinder {}
impl SimpleBinder {
    pub fn bind<T, U>(val: Option<T>, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
        match val {
            Some(v) => f(v),
                None => SimpleBinder::zero(),
        }
    }
    pub fn ret<T>(val: T) -> Option<T> {
        Some(val)
    }
    pub fn zero<T>() -> Option<T> {
        None
    }
}
```
2. And then use your struct as computation expression type in `yer!` macro.
Like this:
```rust
yer! {
    SimpleBinder =>
    let! ...
    let ...
    ret ...
}
```

|yer! expression|description|
| --- | ---|
|$your_struct => | Defines the struct as the provider of computation expression functions. |
|let $your_var = $your_expression|Uses the last defined struct in macros as the provider of `bind` function and calls the `$that_struct::bind($your_expression, &\|$your_var\| { next code })` expression.|
|let $your_var = $your_expression|Defines the variable `$your_var`|
|do! $your_expression|Uses the last defined struct in macros as the provider of `bind` function and calls the `$that_struct::bind($your_expression, &\|_\| { next code })` expression.|
|do $your_expression|Simply runs `$your_expression`|
|ret! $your_expression|Uses the last defined struct in macros as the provider of `ret_from` function and calls the `$that_struct::ret_from($your_expression)` expression.|
|ret $your_expression|Uses the last defined struct in macros as the provider of `ret` function and calls the `$that_struct::ret($your_expression)` expression.|
|yield $your_expression|Uses the last defined struct in macros as the provider of `combine` and `ret_yield` functions and calls the `$that_struct::combine(yer!($that_struct => next code, $that_struct::ret_yield($your_expression))` expression.|
|yield! $your_expression|Works the same as `yield` but uses `$your_struct::ret_yield_from` instead of `ret_yield`|
|if ( $statement ) { $body }|Uses Rust's if-statement as an expression where `$body` is wrapped by `yer!` macro, and `$your_struct::zero()` for else case|
|run $your_struct =>|Uses `$your_struct::run` function by providing last returned value from the CE as an argument for that function|
|$your_struct >>|The same as `run` keyword|



# Examples

For now examples are available in `/tests` directory in repository. GH-link: https://github.com/marat0n/yerevan.rs/blob/dev/tests/common.rs


# Roadmap (+ mini changelog)

__The linked ones are done, they are linked to the crates.io/crates/yerevan page to the version where this roadmap-point was done. Not linked points are the plan for future updates.__

- [0.1](https://crates.io/crates/yerevan/0.1.6)
  - `yer!` macro:
    - `some_struct =>` expression to create the specified structure context where all next expressions of that CE will be executed using methods of this structure (in FP that kind of structures are called [monads](https://en.wikipedia.org/wiki/Monad_(functional_programming)));
    - `let!` expression executed by `bind<T, U>: (val: T, fn: (T) -> U) -> U` method in your defined struct (monad);
    - `let` expression (just define what you want without breaking the CE);
    - `do!` expression executed the same as `let!` but returned value from `bind` method is ignored;
    - `do` expression (just do what you want without breaking the CE);
    - `ret!` expression executed by `ret_from<T>: (val: T) -> T` method in your defined struct (monad);
    - `ret` expression executed by `ret<T, W<T>>: (val: T) -> W<T>` method in your defined struct (monad);
    - `yield` expression executed by `combine<T, W<T>>: (val1: W<T>, val2: T) -> W<T>` where `val1`-parameter is used for all next code in CE and `val2`-parameter is used for executing `ret_yield<T, U>: (val: T) -> U`.
  - initial tests, examples, docs.
- [0.2](https://crates.io/crates/yerevan/0.2.3)
  - [x] upgrade `yer!` macro:
    - [x] add implentation for methods: `Run`, `YieldFrom`, `Zero` from F#'s CE-types;
    - [x] add expressions to macro: `yeild!`, `if ... else`.
        - CAUTION: `else` is not available for now, if you know how to implement it then please create a PR
  - [x] create default CEs for Option and Result types (Railway Execution type).
- 0.3
  - [ ] upgrade `yer!` macro:
    - [ ] add loop-expressions to macro: `for`, `while`, `loop`.
- 0.4
  - [ ] upgrade `yer!` macro:
    - [ ] add match-expression to macro.
- 0.5
  - [ ] add more interesting and usefull CEs.
