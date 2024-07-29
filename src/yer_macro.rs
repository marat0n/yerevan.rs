/// `yerevan.rs` macros for fancy-shmancy syntax for computation expressions using
/// Example:
/// ```rust
/// use yerevan::yerevanize;
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
///     let from_macro = yerevanize!(
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
macro_rules! yerevanize {
    // let!
    (
        $struct_name:ident =>
        let! $var_name:ident = $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::bind($expression, &|$var_name| {
            yerevanize!($struct_name => $($tail)*)
        })
    };

    // do!
    (
        $struct_name:ident =>
        do! $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::bind($expression, &|_| {
            yerevanize!($struct_name => $($tail)*)
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
            (yerevanize!($struct_name => $($tail)*))
        }
    };

    // do
    (
        $struct_name:ident =>
        do $expression:expr;
        $($tail:tt)*
    ) => {
        $expression;
        yerevanize!($struct_name => $($tail)*)
    };
    (
        $struct_name:ident =>
        delay $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::lazy($expression, &|delayed| {
            yerevanize!($struct_name => $($tail)*)
        })
    };
    // return-point
    ( $struct_name:ident => ret $expression:expr ) => {
        $struct_name::ret($expression)
    };
    ( $struct_name:ident => ret! $expression:expr ) => {
        $struct_name::ret_from($expression)
    };
    // changing the CE-functions provider type
    ( $previoues_struct_name:ident => $struct_name:ident => $($tail:tt)* ) => {
        $previoues_struct_name::ret(yerevanize!($struct_name => $($tail)*))
    };
    ( $previoues_struct_name:ident => $struct_name:ident! => $($tail:tt)* ) => {
        $previoues_struct_name::ret_from(yerevanize!($struct_name => $($tail)*))
    };

    // exit-point
    ( $struct_name:ident => ) => { }
}

