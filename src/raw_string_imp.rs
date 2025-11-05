// raw_str::raw_string_imp

use std::{
	borrow::{Borrow, BorrowMut},
	ops::{Deref, DerefMut},
	string::FromUtf8Error,
	fmt,
};

use crate::RawStr;

/// A mutable, growable string that may or may not contain valid UTF-8.
/// 
/// [`RawString`] serves as an alternative to Rust's [`String`] type
/// that allows for arbitrary byte sequences,
/// including those that are not valid UTF-8.
/// 
/// [`RawString`] is implemented as a wrapper around, and implements [`Deref`] + [`DerefMut`] to, [`Vec<u8>`].
/// Therefore, all methods available on [`Vec<u8>`] are also available on [`RawString`].
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RawString(pub Vec<u8>);

impl RawString {
	/// Creates a new, empty [`RawString`].
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::from(Vec::new())
	}

	/// Creates a new [`RawString`] from any type that can be converted into a `Vec<u8>`.
	#[inline]
	#[must_use]
	pub fn from<B>(bytes: B) -> Self
	where
		B: Into<Vec<u8>>
	{
		Self::from_bytes(bytes.into())
	}

	/// Returns a reference to the inner byte slice as a [`RawStr`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub fn as_rawstr(&self) -> &RawStr {
		RawStr::from_bytes(&self.0)
	}

	/// Returns a mutable reference to the inner byte slice as a mutable [`RawStr`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub fn as_mut_rawstr(&mut self) -> &mut RawStr {
		RawStr::from_bytes_mut(&mut self.0)
	}

	/// Wraps the given bytes in a [`RawString`].
	#[doc(hidden)]
	#[inline]
	#[must_use]
	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}

	/// Converts the [`RawString`] into a [`String`] if it contains valid UTF-8.
	/// Returns a [`FromUtf8Error`] if the bytes are not valid UTF-8.
	/// 
	/// See [`String::from_utf8`].
	#[inline]
	#[must_use]
	pub fn to_utf8(self) -> Result<String, FromUtf8Error> {
		String::from_utf8(self.0)
	}

	/// Returns `true` if the [`RawString`] contains valid UTF-8.
	/// 
	/// See [`RawStr::is_utf8`].
	#[inline]
	#[must_use]
	pub fn is_utf8(&self) -> bool {
		self.as_rawstr().is_utf8()
	}
}

impl Deref for RawString {
	type Target = Vec<u8>;
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for RawString {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl AsRef<[u8]> for RawString {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

impl AsRef<RawStr> for RawString {
	#[inline]
	fn as_ref(&self) -> &RawStr {
		self.as_rawstr()
	}
}

impl Borrow<[u8]> for RawString {
	#[inline]
	fn borrow(&self) -> &[u8] {
		&self.0
	}
}

impl Borrow<RawStr> for RawString {
	#[inline]
	fn borrow(&self) -> &RawStr {
		self.as_rawstr()
	}
}

impl AsMut<[u8]> for RawString {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		self.0.as_mut()
	}
}

impl AsMut<RawStr> for RawString {
	#[inline]
	fn as_mut(&mut self) -> &mut RawStr {
		self.as_mut_rawstr()
	}
}

impl BorrowMut<[u8]> for RawString {
	#[inline]
	fn borrow_mut(&mut self) -> &mut [u8] {
		&mut self.0
	}
}

impl BorrowMut<RawStr> for RawString {
	#[inline]
	fn borrow_mut(&mut self) -> &mut RawStr {
		self.as_mut_rawstr()
	}
}

impl fmt::Debug for RawString {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_rawstr().fmt(f)
	}
}

impl fmt::Display for RawString {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_rawstr().fmt(f)
	}
}

impl<T: Into<Vec<u8>>> From<T> for RawString {
	#[inline]
	fn from(value: T) -> Self {
		Self::from(value)
	}
}

impl TryFrom<RawString> for String {
	type Error = FromUtf8Error;

	#[inline]
	fn try_from(this: RawString) -> Result<String, FromUtf8Error> {
		String::from_utf8(this.0)
	}
}