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
