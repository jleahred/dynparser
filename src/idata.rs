#![warn(missing_docs)]

pub(crate) trait IVec<T> {
    fn ipush(self, T) -> Self;
    fn iappend(self, Vec<T>) -> Self;
    fn ipop(self) -> (Option<T>, Self);
}

impl<T> IVec<T> for Vec<T> {
    fn ipush(mut self, v: T) -> Self {
        self.push(v);
        self
    }

    fn iappend(mut self, mut v: Vec<T>) -> Self {
        self.append(&mut v);
        self
    }

    fn ipop(mut self) -> (Option<T>, Self) {
        (self.pop(), self)
    }
}

use std;
pub(crate) fn consume_char(mut chars: std::str::Chars) -> Option<(char, std::str::Chars)> {
    match chars.next() {
        Some(ch) => Some((ch, chars)),
        None => None,
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
