//! Module containing [`crate::railway_exec::RailwayBinder`] struct-ce
//! and [`crate::railway_exec::RailwayExec`] trait.

/// The struct for [`crate::yer`] macro to run railway execution over Result and Option types.
///
/// Raylway execution helps to write less amount of code not when you are worried about all returned
/// Option's Nones and Result's Errs, but when you want to try to execute a bunch of
/// instructions and see if it was executed without problems.
///
/// Example:
/// ```rust
/// use yerevan::{yer, railway_exec::*};
///
/// let railway_positive_scenario = yer! {
///     RailwayBinder =>
///     let! some_one = Result::<i32, ()>::Ok(1);           // stores 1 from Result
///     let! some_two = Result::<i32, String>::Ok(2);       // stores 2 from Result
///     let! some_one_and_two = Some(some_one + some_two);  // wraps 1 + 2 = 3 to Option and unwraps then
///     ret some_one_and_two * 2                            // 3 * 2 = 6 -> Option wrap
/// }; // returns Some(6)
///
/// let railway_negative_scenario = yer! {
///     RailwayBinder =>
///     let! something = Some("something");
///     do! Option::<()>::None; // stop execution here
///     ret something
/// }; // Do not execute "ret" instruction and returns None
/// ```
pub struct RailwayBinder {}

/// Use this trait to implement function `bind` over your struct you want to use in
/// [`crate::railway_exec::RailwayBinder`] CE.
///
/// Example of implementation for Result:
/// ```rust,ignore
/// impl<T, E> RailwayExec<T> for Result<T, E> {
///     fn bind<U>(self, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
///         match self {
///             Ok(val) => f(val),
///             Err(_) => None,
///         }
///     }
/// }
/// ```
///
/// Example of implementation for Option:
/// ```rust,ignore
/// impl<T> RailwayExec<T> for Option<T> {
///     fn bind<U>(self, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
///         match self {
///             Some(val) => f(val),
///             None => None,
///         }
///     }
/// }
/// ```
pub trait RailwayExec<T> {
    fn bind<U>(self, f: &dyn Fn(T) -> Option<U>) -> Option<U>;
}

impl<T, E> RailwayExec<T> for Result<T, E> {
    fn bind<U>(self, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
        match self {
            Ok(val) => f(val),
            Err(_) => None,
        }
    }
}

impl<T> RailwayExec<T> for Option<T> {
    fn bind<U>(self, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
        match self {
            Some(val) => f(val),
            None => None,
        }
    }
}

/// Implementation for simple railway-executions.
impl RailwayBinder {
    pub fn bind<T, U, R: RailwayExec<T>>(val: R, f: &dyn Fn(T) -> Option<U>) -> Option<U> {
        val.bind(f)
    }
    pub fn ret<T>(val: T) -> Option<T> {
        Some(val)
    }
}

