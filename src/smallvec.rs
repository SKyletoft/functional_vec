use smallvec::CollectionAllocErr;

pub trait FunctionalSmallVec {
	type Item;
	fn append_new(self, other: Self) -> Self;
	fn clear_new(self) -> Self;
	fn dedup_new(self) -> Self
	where
		Self::Item: PartialEq<Self::Item>;
	fn dedup_by_new<F>(self, same_bucket: F) -> Self
	where
		F: FnMut(&mut Self::Item, &mut Self::Item) -> bool;
	fn dedup_by_key_new<F, K>(self, key: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> K,
		K: PartialEq<K>;
	fn extend_from_slice_new(self, slice: &[Self::Item]) -> Self
	where
		Self::Item: Copy;
	fn grow_new(self, new_cap: usize) -> Self;
	fn insert_new(self, index: usize, element: Self::Item) -> Self;
	fn insert_from_slice_new(self, index: usize, slice: &[Self::Item]) -> Self
	where
		Self::Item: Copy;
	fn insert_many_new<I>(self, index: usize, iterable: I) -> Self
	where
		I: IntoIterator<Item = Self::Item>;
	fn pop_new(self) -> (Option<Self::Item>, Self);
	fn push_new(self, value: Self::Item) -> Self;
	fn remove_new(self, index: usize) -> (Self::Item, Self);
	fn reserve_new(self, additional: usize) -> Self;
	fn reserve_exact_new(self, additional: usize) -> Self;
	fn resize_new(self, len: usize, value: Self::Item) -> Self
	where
		Self::Item: Clone;
	fn resize_with_new<F>(self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> Self::Item;
	fn retain_new<F>(self, f: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> bool;
	fn retain_mut_new<F>(self, f: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> bool;
	unsafe fn set_len_new(self, new_len: usize) -> Self;
	fn shrink_to_fit_new(self) -> Self;
	fn swap_remove_new(self, index: usize) -> (Self::Item, Self);
	fn truncate_new(self, len: usize) -> Self;
	fn try_grow_new(self, new_cap: usize) -> (Result<(), CollectionAllocErr>, Self);
	fn try_reserve_new(self, additional: usize) -> (Result<(), CollectionAllocErr>, Self);
	fn try_reserve_exact_new(self, additional: usize) -> (Result<(), CollectionAllocErr>, Self);
}

impl<A: smallvec::Array> FunctionalSmallVec for smallvec::SmallVec<A> {
	type Item = A::Item;

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
		Self::Item: PartialEq<Self::Item>,
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

	fn dedup_by_key_new<F, K>(mut self, key: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> K,
		K: PartialEq<K>,
	{
		self.dedup_by_key(key);
		self
	}

	fn extend_from_slice_new(mut self, slice: &[Self::Item]) -> Self
	where
		Self::Item: Copy,
	{
		self.extend_from_slice(slice);
		self
	}

	fn grow_new(mut self, new_cap: usize) -> Self {
		self.grow(new_cap);
		self
	}

	fn insert_new(mut self, index: usize, element: Self::Item) -> Self {
		self.insert(index, element);
		self
	}

	fn insert_from_slice_new(mut self, index: usize, slice: &[Self::Item]) -> Self
	where
		Self::Item: Copy,
	{
		self.insert_from_slice(index, slice);
		self
	}

	fn insert_many_new<I>(mut self, index: usize, iterable: I) -> Self
	where
		I: IntoIterator<Item = A::Item>,
	{
		self.insert_many(index, iterable);
		self
	}

	fn pop_new(mut self) -> (Option<Self::Item>, Self) {
		let res = self.pop();
		(res, self)
	}

	fn push_new(mut self, value: Self::Item) -> Self {
		self.push(value);
		self
	}

	fn remove_new(mut self, index: usize) -> (Self::Item, Self) {
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

	fn resize_new(mut self, len: usize, value: Self::Item) -> Self
	where
		Self::Item: Clone,
	{
		self.resize(len, value);
		self
	}

	fn resize_with_new<F>(mut self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> Self::Item,
	{
		self.resize_with(new_len, f);
		self
	}

	fn retain_new<F>(mut self, f: F) -> Self
	where
		F: FnMut(&mut Self::Item) -> bool,
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

	fn shrink_to_fit_new(mut self) -> Self {
		self.shrink_to_fit();
		self
	}

	fn swap_remove_new(mut self, index: usize) -> (Self::Item, Self) {
		let res = self.swap_remove(index);
		(res, self)
	}

	fn truncate_new(mut self, len: usize) -> Self {
		self.truncate(len);
		self
	}

	fn try_grow_new(mut self, new_cap: usize) -> (Result<(), CollectionAllocErr>, Self) {
		let res = self.try_grow(new_cap);
		(res, self)
	}

	fn try_reserve_new(mut self, additional: usize) -> (Result<(), CollectionAllocErr>, Self) {
		let res = self.try_reserve(additional);
		(res, self)
	}

	fn try_reserve_exact_new(
		mut self,
		additional: usize,
	) -> (Result<(), CollectionAllocErr>, Self) {
		let res = self.try_reserve_exact(additional);
		(res, self)
	}
}
