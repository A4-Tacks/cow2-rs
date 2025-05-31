#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

use core::{borrow::Borrow, ops::Deref};

use alloc::borrow::ToOwned;

mod like_impls;

/// Auxiliary enum of [`Cow`]
pub enum CowT<'a, B: ?Sized + ToOwned, O> {
    Borrowed(&'a B),
    Owned(O),
}

/// Like [`borrow::Cow`](alloc::borrow::Cow), but `T` is covariant
///
/// # Examples
///
/// ```
/// use cow2::Cow;
///
/// fn foo<'a>(_: Cow<'a, [Cow<'a, str>]>) { }
/// fn bar<'a>(x: Cow<'a, [Cow<'static, str>]>) {
///     foo(x);
/// }
/// ```
pub type Cow<'a, T> = CowT<'a, T, <T as ToOwned>::Owned>;

impl<B: ?Sized + ToOwned> Cow<'_, B> {
    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow2::Cow;
    ///
    /// let mut cow = Cow::Borrowed("foo");
    /// cow.to_mut().make_ascii_uppercase();
    ///
    /// assert_eq!(
    ///   cow,
    ///   Cow::Owned(String::from("FOO")) as Cow<'_, str>
    /// );
    /// ```
    pub fn to_mut(&mut self) -> &mut B::Owned {
        match self {
            CowT::Borrowed(borrowed) => {
                *self = Self::Owned(borrowed.to_owned());
                match self {
                    CowT::Borrowed(_) => unreachable!(),
                    CowT::Owned(owned) => owned,
                }
            },
            CowT::Owned(owned) => owned,
        }
    }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    ///
    /// # Examples
    ///
    /// Calling `into_owned` on a `Cow::Borrowed` returns a clone of the borrowed data:
    ///
    /// ```
    /// use cow2::Cow;
    ///
    /// let s = "Hello world!";
    /// let cow = Cow::Borrowed(s);
    ///
    /// assert_eq!(
    ///   cow.into_owned(),
    ///   String::from(s)
    /// );
    /// ```
    ///
    /// Calling `into_owned` on a `Cow::Owned` returns the owned data. The data is moved out of the
    /// `Cow` without being cloned.
    ///
    /// ```
    /// use cow2::Cow;
    ///
    /// let s = "Hello world!";
    /// let cow: Cow<'_, str> = Cow::Owned(String::from(s));
    ///
    /// assert_eq!(
    ///     cow.into_owned(),
    ///     String::from(s)
    /// );
    /// ```
    pub fn into_owned(self) -> B::Owned {
        match self {
            CowT::Borrowed(borrowed) => borrowed.to_owned(),
            CowT::Owned(owned) => owned,
        }
    }
}

impl<B: ?Sized + ToOwned, O: Clone> Clone for CowT<'_, B, O> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(borrowed) => Self::Borrowed(borrowed),
            Self::Owned(owned) => Self::Owned(owned.clone()),
        }
    }
}

impl<B: ?Sized + ToOwned, O: Borrow<B>> Deref for CowT<'_, B, O> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        match self {
            CowT::Borrowed(borrowed) => borrowed,
            CowT::Owned(owned) => owned.borrow(),
        }
    }
}
