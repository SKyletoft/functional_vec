extern crate alloc;

use alloc::collections::TryReserveError;

pub trait FunctionalVecDeque {
	type Item;

	fn append_new(self, other: Self) -> Self;
	fn insert_new(self, index: usize, value: Self::Item) -> Self;
	fn make_contiguous_new(self) -> Self;
	fn pop_back_new(self) -> (Option<Self::Item>, Self);
	fn pop_front_new(self) -> (Option<Self::Item>, Self);
	fn push_back_new(self, value: Self::Item) -> Self;
	fn push_front_new(self, value: Self::Item) -> Self;
	fn remove_new(self, index: usize) -> (Option<Self::Item>, Self);
	fn reserve_new(self, additional: usize) -> Self;
	fn reserve_exact_new(self, additional: usize) -> Self;
	fn resize_new(self, new_len: usize, value: Self::Item) -> Self
	where
		Self::Item: Clone;
	fn resize_with_new(self, new_len: usize, generator: impl FnMut() -> Self::Item) -> Self;
	fn retain_new<F>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> bool;
	fn retain_mut_new<F>(self, f: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> bool;
	fn rotate_left_new(self, mid: usize) -> Self;
	fn rotate_right_new(self, mid: usize) -> Self;
	fn shrink_to_new(self, min_capacity: usize) -> Self;
	fn shrink_to_fit_new(self) -> Self;
	fn split_off_new(self, at: usize) -> (Self, Self)
	where
		Self: Sized;
	fn swap_new(self, i: usize, j: usize) -> Self;
	fn swap_remove_back_new(self, index: usize) -> (Option<Self::Item>, Self);
	fn swap_remove_front_new(self, index: usize) -> (Option<Self::Item>, Self);
	fn truncate_new(self, len: usize) -> Self;
	fn try_reserve_new(self, additional: usize) -> (Result<(), TryReserveError>, Self);
	fn try_reserve_exact_new(self, additional: usize) -> (Result<(), TryReserveError>, Self);
}

impl<T> FunctionalVecDeque for alloc::collections::VecDeque<T> {
	type Item = T;

	fn append_new(mut self, mut other: Self) -> Self {
		self.append(&mut other);
		self
	}

	fn insert_new(mut self, index: usize, value: Self::Item) -> Self {
		self.insert(index, value);
		self
	}

	fn make_contiguous_new(mut self) -> Self {
		self.make_contiguous();
		self
	}

	fn pop_back_new(mut self) -> (Option<Self::Item>, Self) {
		let res = self.pop_back();
		(res, self)
	}

	fn pop_front_new(mut self) -> (Option<Self::Item>, Self) {
		let res = self.pop_front();
		(res, self)
	}

	fn push_back_new(mut self, value: Self::Item) -> Self {
		self.push_back(value);
		self
	}

	fn push_front_new(mut self, value: Self::Item) -> Self {
		self.push_front(value);
		self
	}

	fn remove_new(mut self, index: usize) -> (Option<Self::Item>, Self) {
		let res = self.remove(index);
		(res, self)
	}

	fn reserve_new(mut self, additional: usize) -> Self {
		self.reserve(additional);
		self
	}

	fn reserve_exact_new(mut self, additional: usize) -> Self {
		self.reserve_exact(additional);
		self
	}

	fn resize_new(mut self, new_len: usize, value: Self::Item) -> Self
	where
		Self::Item: Clone,
	{
		self.resize(new_len, value);
		self
	}

	fn resize_with_new(mut self, new_len: usize, generator: impl FnMut() -> Self::Item) -> Self {
		self.resize_with(new_len, generator);
		self
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

	fn rotate_left_new(mut self, mid: usize) -> Self {
		self.rotate_left(mid);
		self
	}

	fn rotate_right_new(mut self, mid: usize) -> Self {
		self.rotate_right(mid);
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

	fn split_off_new(mut self, at: usize) -> (Self, Self)
	where
		Self: Sized,
	{
		let res = self.split_off(at);
		(res, self)
	}

	fn swap_new(mut self, i: usize, j: usize) -> Self {
		self.swap(i, j);
		self
	}

	fn swap_remove_back_new(mut self, index: usize) -> (Option<Self::Item>, Self) {
		let res = self.swap_remove_back(index);
		(res, self)
	}

	fn swap_remove_front_new(mut self, index: usize) -> (Option<Self::Item>, Self) {
		let res = self.swap_remove_front(index);
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
