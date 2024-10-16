#[cfg(test)]
mod tests {
    use std::{fmt::Display, i32};

    use yerevan::yer;

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
        pub fn ret_yield<T>(val: T) -> Option<Vec<T>> {
            Some(vec![val])
        }
        pub fn combine<T>(val1: Option<Vec<T>>, val2: Option<Vec<T>>) -> Option<Vec<T>> {
            match (val1, val2) {
                (Some(mut unwrapped1), Some(mut unwrapped2)) => {
                    unwrapped2.append(&mut unwrapped1);
                    Some(unwrapped2)
                }
                _ => None,
            }
        }
        pub fn run<T>(val: Option<T>) -> String
        where
            T: Display,
        {
            match val {
                Some(v) => format!("SOME: {}", v.to_string()),
                None => "NONE".to_string(),
            }
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

    #[test]
    fn check_binding_for_simplebinder() {
        let value_from_yer_macro = yer!(
            SimpleBinder =>
            let! unwrapper1 = Some(1);
            let! unwrapped2 = Some(2);
            ret unwrapped2 + unwrapper1
        );

        assert_eq!(
            value_from_yer_macro,
            SimpleBinder::bind(Some(1), &|unwrapped1| {
                SimpleBinder::bind(Some(2), &|unwrapped2| {
                    SimpleBinder::ret(unwrapped2 + unwrapped1)
                })
            }),
            "Testing macro is returning the same value as the same non-yerevanized expression"
        );

        assert_eq!(
            value_from_yer_macro,
            Some(3),
            "Testing specific tested yer-macro is returning Some(3) from the sum of binded Some(1) and binded Some(2)"
        );
    }

    #[test]
    fn check_binding_for_incrementer() {
        let value_from_yer_macro = yer!(
            Incrementer =>
            let! unwrapper1 = 1; // incrementing to 2
            let! unwrapped2 = 2; // incrementing to 3
            ret unwrapped2 + unwrapper1 // returning 5
        );

        assert_eq!(
            value_from_yer_macro.clone(),
            Incrementer::bind(1, &|unwrapped1| {
                Incrementer::bind(2, &|unwrapped2| Incrementer::ret(unwrapped2 + unwrapped1))
            }),
            "Testing macro is returning the same value as the same non-yerevanized expression"
        );

        assert_eq!(
            value_from_yer_macro, 5,
            "Testing specific tested yer-macro is returning 5 from the sum of binded 1 and binded 2"
        );
    }

    #[test]
    fn check_zero() {
        assert_eq!(
            yer!(
                SimpleBinder =>
                do! None::<()>;
                ret ()
            ),
            None,
            "Testing not successful returning"
        );
    }

    #[test]
    fn check_yielding() {
        let value_from_yer_macro = yer!(
            SimpleBinder =>
            let! some_value = Some(1);
            yield some_value; // yielding 1
            yield 2; // yielding 2
            let some_sum = some_value + 2; // result is 3
            yield some_sum; // yielding 3
        );

        assert_eq!(
            value_from_yer_macro,
            Some(vec![1, 2, 3]),
            "Testing yielding values to one-dimensional vec"
        );
    }

    #[test]
    fn check_ce_nesting() {
        let value_from_yer_macro = yer!(
            SimpleBinder =>
            let! unwrapped1 = Some(2); // 2
            let! unwrapped2 = Some(3); // 3
            let one = 1;
            Incrementer =>
            let! res = one + unwrapped1 + unwrapped2; // incremented by 1 result of 1 + 2 + 3 = 6 + 1 = 7
            ret res // Some(7)
        );

        assert_eq!(
            value_from_yer_macro,
            SimpleBinder::bind(Some(2), &|unwrapped1| {
                SimpleBinder::bind(Some(3), &|unwrapped2| {
                    let one = 1;
                    SimpleBinder::ret(Incrementer::bind(one + unwrapped1 + unwrapped2, &|res| {
                        Incrementer::ret(res)
                    }))
                })
            }),
            "Testing macro is returning the same value as the same non-yerevanized expression"
        );

        assert_eq!(
            value_from_yer_macro,
            Some(7),
            "Testing specific tested yer-macro is returning a correct result"
        );
    }

    #[test]
    fn check_ce_run() {
        let value_from_yer_macro1 = yer!(
            run SimpleBinder =>
            let! one = Some(1);
            Incrementer =>
            let! two = one;
            ret two
        );

        let value_from_yer_macro2 = yer!(
            run SimpleBinder =>
            let! some_none = Option::<i32>::None;
            ret some_none
        );

        assert_eq!(
            value_from_yer_macro1,
            SimpleBinder::run(SimpleBinder::bind(Some(1), &|one| {
                SimpleBinder::ret(Incrementer::bind(one, &|two| Incrementer::ret(two)))
            })),
            "1) Testing macro is returning the same value as the same non-yerevanized expression"
        );

        assert_eq!(
            value_from_yer_macro2,
            SimpleBinder::run(SimpleBinder::bind(Option::<i32>::None, &|hello| {
                SimpleBinder::ret(hello)
            })),
            "2) Testing macro is returning the same value as the same non-yerevanized expression"
        );

        assert_eq!(
            value_from_yer_macro1, "SOME: 2",
            "1) Testing specific tested yer-macro is returning a correct result"
        );

        assert_eq!(
            value_from_yer_macro2, "NONE",
            "2) Testing specific tested yer-macro is returning a correct result"
        );
    }
}
