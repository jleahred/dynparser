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
