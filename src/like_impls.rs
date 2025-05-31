#![allow(clippy::partialeq_ne_impl)]

use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign},
};

use alloc::{
    borrow::{self as b, ToOwned},
    string::String,
};

use crate::Cow;


impl<'a> Add<&'a str> for Cow<'a, str> {
    type Output = Self;

    fn add(mut self, rhs: &'a str) -> Self::Output {
        self += rhs;
        self
    }
}


impl<'a> Add<Cow<'a, str>> for Cow<'a, str> {
    type Output = Self;

    fn add(mut self, rhs: Cow<'a, str>) -> Self::Output {
        self += rhs;
        self
    }
}


impl<'a> Add<b::Cow<'a, str>> for Cow<'a, str> {
    type Output = Self;

    fn add(mut self, rhs: b::Cow<'a, str>) -> Self::Output {
        self += rhs;
        self
    }
}


impl<'a> Add<Cow<'a, str>> for b::Cow<'a, str> {
    type Output = Self;

    fn add(mut self, rhs: Cow<'a, str>) -> Self::Output {
        self += rhs;
        self
    }
}


impl<'a> AddAssign<&'a str> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: &'a str) {
        if self.is_empty() {
            *self = Self::Borrowed(rhs)
        } else if !rhs.is_empty() {
            if let Self::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = s.into();
            }
            self.to_mut().push_str(rhs);
        }
    }
}


impl<'a> AddAssign<Cow<'a, str>> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: Cow<'a, str>) {
        if self.is_empty() {
            *self = rhs
        } else if !rhs.is_empty() {
            if let Self::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = s.into();
            }
            self.to_mut().push_str(&rhs);
        }
    }
}


impl<'a> AddAssign<Cow<'a, str>> for b::Cow<'a, str> {
    fn add_assign(&mut self, rhs: Cow<'a, str>) {
        if self.is_empty() {
            *self = rhs.into()
        } else if !rhs.is_empty() {
            if let Self::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = s.into();
            }
            self.to_mut().push_str(&rhs);
        }
    }
}


impl<'a> AddAssign<b::Cow<'a, str>> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: b::Cow<'a, str>) {
        if self.is_empty() {
            *self = rhs.into()
        } else if !rhs.is_empty() {
            if let Self::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = s.into();
            }
            self.to_mut().push_str(&rhs);
        }
    }
}


impl<T> AsRef<T> for Cow<'_, T>
where T: ToOwned + ?Sized,
{
    fn as_ref(&self) -> &T {
        self
    }
}


#[cfg(feature = "std")]
impl AsRef<std::path::Path> for Cow<'_, std::ffi::OsStr> {
    fn as_ref(&self) -> &std::path::Path {
        (**self).as_ref()
    }
}


impl<T> Borrow<T> for Cow<'_, T>
where T: ToOwned + ?Sized,
{
    fn borrow(&self) -> &T {
        self
    }
}


impl<T> Debug for Cow<'_, T>
where T: ToOwned + ?Sized + Debug,
      T::Owned: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Cow::Borrowed(borrowed) => borrowed.fmt(f),
            Cow::Owned(owned) => owned.fmt(f),
        }
    }
}


impl<T> Default for Cow<'_, T>
where T: ToOwned + ?Sized,
      T::Owned: Default,
{
    fn default() -> Self {
        Self::Owned(Default::default())
    }
}


impl<T> Display for Cow<'_, T>
where T: ToOwned + ?Sized + Display,
      T::Owned: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Cow::Borrowed(borrowed) => borrowed.fmt(f),
            Cow::Owned(owned) => owned.fmt(f),
        }
    }
}


impl<T> Eq for Cow<'_, T>
where T: ToOwned + ?Sized + Eq,
{
}


impl<T> Ord for Cow<'_, T>
where T: ToOwned + ?Sized + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(other)
    }
}


impl<B> PartialEq for Cow<'_, B>
where B: ToOwned + ?Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }

    fn ne(&self, other: &Self) -> bool {
        **self != **other
    }
}


impl<B> PartialOrd for Cow<'_, B>
where B: ToOwned + ?Sized + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    fn lt(&self, other: &Self) -> bool {
        (**self).lt(&**other)
    }

    fn le(&self, other: &Self) -> bool {
        (**self).le(&**other)
    }

    fn gt(&self, other: &Self) -> bool {
        (**self).gt(&**other)
    }

    fn ge(&self, other: &Self) -> bool {
        (**self).ge(&**other)
    }
}


impl<T, B> PartialEq<T> for Cow<'_, B>
where B: ToOwned + ?Sized,
      for<'a> b::Cow<'a, B>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        b::Cow::Borrowed(&**self) == *other
    }

    fn ne(&self, other: &T) -> bool {
        b::Cow::Borrowed(&**self) != *other
    }
}


impl<T, B> PartialOrd<T> for Cow<'_, B>
where B: ToOwned + ?Sized,
      for<'a> b::Cow<'a, B>: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        b::Cow::Borrowed(&**self).partial_cmp(other)
    }

    fn lt(&self, other: &T) -> bool {
        b::Cow::Borrowed(&**self).lt(other)
    }

    fn le(&self, other: &T) -> bool {
        b::Cow::Borrowed(&**self).le(other)
    }

    fn gt(&self, other: &T) -> bool {
        b::Cow::Borrowed(&**self).gt(other)
    }

    fn ge(&self, other: &T) -> bool {
        b::Cow::Borrowed(&**self).ge(other)
    }
}


impl<'a> Extend<Cow<'a, str>> for String {
    fn extend<T: IntoIterator<Item = Cow<'a, str>>>(&mut self, iter: T) {
        iter.into_iter().for_each(|s| self.push_str(&s));
    }
}


#[cfg(feature = "std")]
impl<'a> Extend<Cow<'a, std::ffi::OsStr>> for std::ffi::OsString {
    fn extend<T: IntoIterator<Item = Cow<'a, std::ffi::OsStr>>>(&mut self, iter: T) {
        iter.into_iter().for_each(|s| self.push(&*s));
    }
}


impl<'a, T, B> From<T> for Cow<'a, B>
where B: ToOwned + ?Sized,
      b::Cow<'a, B>: From<T>,
{
    fn from(value: T) -> Self {
        match value.into() {
            b::Cow::Borrowed(borrowed) => Self::Borrowed(borrowed),
            b::Cow::Owned(owned) => Self::Owned(owned),
        }
    }
}


#[allow(clippy::from_over_into)]
impl<'a, B> Into<b::Cow<'a, B>> for Cow<'a, B>
where B: ToOwned + ?Sized,
{
    fn into(self) -> b::Cow<'a, B> {
        match self {
            Cow::Borrowed(borrowed) => b::Cow::Borrowed(borrowed),
            Cow::Owned(owned) => b::Cow::Owned(owned),
        }
    }
}


impl<'a, T, B> FromIterator<T> for Cow<'a, B>
where B: ToOwned + ?Sized,
      b::Cow<'a, B>: FromIterator<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        b::Cow::from_iter(iter).into()
    }
}


impl<B> Hash for Cow<'_, B>
where B: ToOwned + ?Sized + Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}
