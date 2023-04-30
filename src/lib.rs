#![no_std]

#[cfg(feature = "vec")]
mod vec;
#[cfg(feature = "vec")]
pub use vec::FunctionalVec;

#[cfg(feature = "vecdeque")]
mod vecdeque;
#[cfg(feature = "vecdeque")]
pub use vecdeque::FunctionalVecDeque;

#[cfg(feature = "smallvec")]
mod smallvec;
#[cfg(feature = "smallvec")]
pub use smallvec::FunctionalSmallVec;
