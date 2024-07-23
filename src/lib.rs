/// `yerevan.rs` macros for fancy-shmancy syntax for computation expressions using
/// Example:
/// ```rust
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
/// pub fn showcase(wrapped1: Option<i32>, wrapper2: Option<&str>) -> Option<i32> {
///     yerevanize!(
///         SimpleBinder =>
///         let! unwrapped1 = wrapped1;
///         let! unwrapped2 = wrapper2;
///         let one = 1;
///         Incrementer =>
///         let! res = one + unwrapped1 + (unwrapped2.len() as i32);
///         ret res
///     )
/// }
/// ```
#[allow(unused_macros)]
macro_rules! yerevanize {
    // value-binding
    (
        $struct_name:ident =>
        let! $var_name:ident = $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::bind($expression, &|$var_name| {
            yerevanize!($struct_name => $($tail)*)
        })
    };
    (
        $struct_name:ident =>
        do! $expression:expr;
        $($tail:tt)*
    ) => {
        $struct_name::bind($expression, &|_| {
            yerevanize!($struct_name => $($tail)*)
        })
    };
    // letting to just execute basic expressions
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


// tests :)
#[cfg(test)]
mod tests {
    use std::i32;
    use std::str;

    // Some simple user-defined structs for compuation expressions
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

    struct Incrementer {}
    impl Incrementer {
        pub fn bind(val: i32, f: &dyn Fn(i32) -> i32) -> i32 {
            f(val + 1)
        }
        pub fn ret(val: i32) -> i32 {
            val
        }
    }

    pub fn showcase(wrapped1: Option<i32>, wrapper2: Option<&str>, for_do: Option<()>) -> Option<i32> {
        yerevanize!(
            SimpleBinder =>
            let! v = wrapped1;
            let a = 1;
            let! w = wrapper2;
            do! for_do;
            Incrementer =>
            let! x = a + v + (w.len() as i32);
            ret x
        )
    }

    #[test]
    fn check() {
        assert_eq!(showcase(Some(5), Some(&"something"), Some(())), Some(16));
    }

    #[test]
    fn check2() {
        assert_eq!(showcase(Some(1), None, Some(())), None);
    }

    #[test]
    fn check3() {
        assert_eq!(showcase(Some(1), Some(&"something"), None), None);
    }
}
