extern crate alloc;

pub trait FunctionalVecDeque {}

impl<T> FunctionalVecDeque for alloc::collections::VecDeque<T> {}
