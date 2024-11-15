#[cfg(test)]
mod tests {
    use yerevan::{railway_exec::*, yer};

    #[test]
    fn check_railway_exec() {
        let value_from_macro_with_railway = yer! {
            RailwayBinder =>
            let! some_one = Result::<i32, ()>::Ok(1);
            let! some_two = Result::<i32, String>::Ok(2);
            let! some_one_and_two = Some(some_one + some_two);
            ret some_one_and_two * 2
        };

        let none_value_from_macro_with_railway = yer! {
            RailwayBinder =>
            let! something = Some("something");
            do! Option::<()>::None;
            ret something
        };

        assert_eq!(
            value_from_macro_with_railway,
            Some(6),
            "Check that railway execution works correctly on positive scenario."
        );

        assert_eq!(
            none_value_from_macro_with_railway,
            Option::<&str>::None,
            "Check that railway execution works correctly on negative scenario."
        );
    }
}
