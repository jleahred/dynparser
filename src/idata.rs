#![warn(missing_docs)]

use std;

pub(crate) trait IVec<T> {
    fn ipush(self, T) -> Self;
}

impl<T> IVec<T> for Vec<T>
where
    T: std::fmt::Debug,
{
    fn ipush(mut self, v: T) -> Self {
        self.push(v);
        self
    }
}

//-----------------------------------------------------------------------
//  TailCall
//-----------------------------------------------------------------------
pub(crate) enum TailCall<T, R> {
    Call(T),
    Return(R),
}

pub(crate) fn tail_call<T, R, F>(seed: T, recursive_function: F) -> R
where
    F: Fn(T) -> TailCall<T, R>,
{
    let mut state = TailCall::Call(seed);
    loop {
        match state {
            TailCall::Call(arg) => {
                state = recursive_function(arg);
            }
            TailCall::Return(result) => {
                return result;
            }
        }
    }
}
