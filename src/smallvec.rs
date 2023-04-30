pub trait FunctionalSmallVec {}

impl<A: smallvec::Array> FunctionalSmallVec for smallvec::SmallVec<A> {}
