//! Module containing the [`crate::yer`] macro.

/// `yerevan.rs` macros for fancy-shmancy syntax for computation expressions.
///
/// Example:
/// ```rust
/// use yerevan::yer;
///
/// // Some simple user-defined structs for compuation expressions
/// struct SimpleBinder {}
/// impl SimpleBinder {
///     pub fn bind<T, U>(val: Option<T>, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
///         match val {
///             Some(v) => f(v),
///             None => SimpleBinder::zero(),
///         }
///     }
///     pub fn ret<T>(val: T) -> Option<T> {
///         Some(val)
///     }
///     pub fn zero<T>() -> Option<T> {
///         None
///     }
/// }
///
/// struct Incrementer {}
/// impl Incrementer {
///     pub fn bind(val: i32, f: &dyn Fn(i32) -> i32) -> i32 {
///         f(val + 1)
///     }
///     pub fn ret(val: i32) -> i32 {
///         val
///     }
/// }
///
/// pub fn showcase(wrapped1: Option<i32>, wrapped2: Option<&str>) -> bool {
///     let from_macro = yer!(
///         SimpleBinder =>
///         let! unwrapped1 = wrapped1;
///         let! unwrapped2 = wrapped2;
///         let one = 1;
///         Incrementer =>
///         let! res = one + unwrapped1 + (unwrapped2.len() as i32);
///         ret res
///     );
///     let by_hand =
///         SimpleBinder::bind(
///         wrapped1, &|unwrapped1| {
///         SimpleBinder::bind(
///         wrapped2, &|unwrapped2| {
///         let one = 1;
///         SimpleBinder::ret(
///         Incrementer::bind(
///         one + unwrapped1 + (unwrapped2.len() as i32), &|res| {
///         Incrementer::ret(res)
///         })
///         )
///         })
///         });
///     from_macro == by_hand // true
/// }
/// ```

#[allow(unused_macros)]
#[macro_export]
macro_rules! yer {
    // let!
    (
        $struct_name:ident =>
        let! $var_name:ident = $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::bind($expression, &|$var_name| {
            yer!($struct_name => $($tail)*)
        })
    };

    // do!
    (
        $struct_name:ident =>
        do! $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::bind($expression, &|_| {
            yer!($struct_name => $($tail)*)
        })
    };

    // let
    (
        $struct_name:ident =>
        let $var_name:ident = $expression:expr;
        $($tail:tt)*
    ) => {
        {
            let $var_name = $expression;
            (yer!($struct_name => $($tail)*))
        }
    };

    // do
    (
        $struct_name:ident =>
        do $expression:expr;
        $($tail:tt)*
    ) => {
        $expression;
        yer!($struct_name => $($tail)*)
    };

    // delay
    // (
    //     $struct_name:ident =>
    //     delay $expression:expr;
    //     $($tail:tt)*
    // ) => {
    //     $struct_name::delay($expression, &|delayed| {
    //         yer!($struct_name => $($tail)*)
    //     })
    // };

    // ret
    ( $struct_name:ident => ret $expression:expr ) => {
        $struct_name::ret($expression)
    };

    // ret!
    ( $struct_name:ident => ret! $expression:expr ) => {
        $struct_name::ret_from($expression)
    };

    // yield as return (last yield)
    (
        $struct_name:ident =>
        yield $expression:expr;
    ) => {
        $struct_name::ret_yield($expression)
    };

    // yield with combiner
    (
        $struct_name:ident =>
        yield $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::combine(
            yer!($struct_name => $($tail)*),
            $struct_name::ret_yield($expression)
        )
    };

    // changing the CE-functions provider type
    ( $previous_struct_name:ident => $struct_name:ident => $($tail:tt)* ) => {
        $previous_struct_name::ret(yer!($struct_name => $($tail)*))
    };
    ( $previous_struct_name:ident => $struct_name:ident! => $($tail:tt)* ) => {
        $previous_struct_name::ret_from(yer!($struct_name => $($tail)*))
    };

    // exit-point
    ( $struct_name:ident => ) => { }
}
