extern crate alloc;

use alloc::collections::TryReserveError;
use core::ops::RangeBounds;

pub trait FunctionalVec
where
	Self: Sized,
{
	type Item;
	fn append_new(self, other: Self) -> Self;
	fn clear_new(self) -> Self;
	fn dedup_new(self) -> Self
	where
		Self::Item: PartialEq;
	fn dedup_by_new<F>(self, same_bucket: F) -> Self
	where
		F: FnMut(&mut Self::Item, &mut Self::Item) -> bool;
	fn dedup_by_key_new<F, K>(self, same_bucket: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> K,
		K: PartialEq<K>;
	fn extend_from_slice_new(self, other: &[Self::Item]) -> Self
	where
		Self::Item: Clone;
	fn extend_from_within_new<R>(self, src: R) -> Self
	where
		Self::Item: Clone,
		R: RangeBounds<usize>;
	fn insert_new(self, index: usize, element: Self::Item) -> Self;
	fn remove_new(self, index: usize) -> (Self::Item, Self);
	fn resize_new(self, new_len: usize, value: Self::Item) -> Self
	where
		Self::Item: Clone;
	fn resize_with_new<F>(self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> Self::Item;
	fn reserve_new(self, additional: usize) -> Self;
	fn reserve_exact_new(self, additional: usize) -> Self;
	fn push_new(self, item: Self::Item) -> Self;
	fn pop_new(self) -> (Option<Self::Item>, Self);
	fn retain_new<F>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> bool;
	fn retain_mut_new<F>(self, f: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> bool;
	unsafe fn set_len_new(self, new_len: usize) -> Self;
	fn shrink_to_new(self, min_capacity: usize) -> Self;
	fn shrink_to_fit_new(self) -> Self;
	fn split_off_new(self, at: usize) -> (Self, Self);
	fn swap_remove_new(self, index: usize) -> (Self::Item, Self);
	fn truncate_new(self, len: usize) -> Self;
	fn try_reserve_new(self, additional: usize) -> (Result<(), TryReserveError>, Self);
	fn try_reserve_exact_new(self, additional: usize) -> (Result<(), TryReserveError>, Self);
}

impl<T: Sized> FunctionalVec for alloc::vec::Vec<T> {
	type Item = T;

	fn append_new(mut self, mut other: Self) -> Self {
		self.append(&mut other);
		self
	}

	fn clear_new(mut self) -> Self {
		self.clear();
		self
	}

	fn dedup_new(mut self) -> Self
	where
		T: PartialEq,
	{
		self.dedup();
		self
	}

	fn dedup_by_new<F>(mut self, same_bucket: F) -> Self
	where
		F: FnMut(&mut Self::Item, &mut Self::Item) -> bool,
	{
		self.dedup_by(same_bucket);
		self
	}

	fn dedup_by_key_new<F, K>(mut self, same_bucket: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> K,
		K: PartialEq<K>,
	{
		self.dedup_by_key(same_bucket);
		self
	}

	fn extend_from_slice_new(mut self, other: &[Self::Item]) -> Self
	where
		T: Clone,
	{
		self.extend_from_slice(other);
		self
	}

	fn extend_from_within_new<R>(mut self, src: R) -> Self
	where
		T: Clone,
		R: RangeBounds<usize>,
	{
		self.extend_from_within(src);
		self
	}

	fn insert_new(mut self, index: usize, element: Self::Item) -> Self {
		self.insert(index, element);
		self
	}

	fn remove_new(mut self, index: usize) -> (Self::Item, Self) {
		let res = self.remove(index);
		(res, self)
	}

	fn resize_new(mut self, new_len: usize, value: Self::Item) -> Self
	where
		Self::Item: Clone,
	{
		self.resize(new_len, value);
		self
	}

	fn resize_with_new<F>(mut self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> Self::Item,
	{
		self.resize_with(new_len, f);
		self
	}

	fn reserve_new(mut self, additional: usize) -> Self {
		self.reserve(additional);
		self
	}

	fn reserve_exact_new(mut self, additional: usize) -> Self {
		self.reserve_exact(additional);
		self
	}

	/// ```
	/// use functional_vec::FunctionalVec;
	///
	/// let v = (0..10).fold(Vec::new(), |acc, curr| acc.push_new(curr));
	/// assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	/// ```
	fn push_new(mut self, item: Self::Item) -> Self {
		self.push(item);
		self
	}

	fn pop_new(mut self) -> (Option<Self::Item>, Self) {
		let res = self.pop();
		(res, self)
	}

	fn retain_new<F>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> bool,
	{
		self.retain(f);
		self
	}

	fn retain_mut_new<F>(mut self, f: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> bool,
	{
		self.retain_mut(f);
		self
	}

	unsafe fn set_len_new(mut self, new_len: usize) -> Self {
		self.set_len(new_len);
		self
	}

	fn shrink_to_new(mut self, min_capacity: usize) -> Self {
		self.shrink_to(min_capacity);
		self
	}

	fn shrink_to_fit_new(mut self) -> Self {
		self.shrink_to_fit();
		self
	}

	fn split_off_new(mut self, at: usize) -> (Self, Self) {
		let res = self.split_off(at);
		(res, self)
	}

	fn swap_remove_new(mut self, index: usize) -> (Self::Item, Self) {
		let res = self.swap_remove(index);
		(res, self)
	}

	fn truncate_new(mut self, len: usize) -> Self {
		self.truncate(len);
		self
	}

	fn try_reserve_new(mut self, additional: usize) -> (Result<(), TryReserveError>, Self) {
		let res = self.try_reserve(additional);
		(res, self)
	}

	fn try_reserve_exact_new(mut self, additional: usize) -> (Result<(), TryReserveError>, Self) {
		let res = self.try_reserve_exact(additional);
		(res, self)
	}
}
