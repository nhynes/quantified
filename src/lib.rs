#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quantified<T> {
    None,
    Some(T),
    Excluding(T),
    All,
}

impl<T> Quantified<T> {
    /// Maps an `Quantified<T>` to `Quantified<U>` by applying a function to a contained `Some`
    /// or `Excluding` value.
    ///
    /// # Examples
    ///
    /// Converts an `Quantified<`[`String`]`>` into an `Quantified<`[`usize`]`>`, consuming
    /// the original:
    ///
    /// [`String`]: ../../std/string/struct.String.html
    /// ```
    /// # use quantified::Quantified;
    /// let some_string = Quantified::Some(String::from("Hello, World!"));
    /// // `Quantified::map` takes self *by value*, consuming `maybe_some_string`
    /// let some_len = some_string.map(|s| s.len());
    ///
    /// assert_eq!(some_len, Quantified::Some(13));
    /// ```
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Quantified<U> {
        match self {
            Self::Some(x) => Quantified::Some(f(x)),
            Self::Excluding(x) => Quantified::Excluding(f(x)),
            Self::None => Quantified::None,
            Self::All => Quantified::All,
        }
    }

    /// Converts from `&Quantified<T>` to `Quantified<&T>`.
    ///
    /// # Examples
    ///
    /// Converts a `Quantified<`[`String`]`>` into a `Quantified<`[`usize`]`>`,
    /// preserving the original.
    /// The [`map`] method takes the `self` argument by value, consuming the original,
    /// so this technique uses `as_ref` to first take a `Quantified` to a reference
    /// to the value inside the original.
    ///
    /// [`map`]: Quantified::map
    /// [`String`]: ../../std/string/struct.String.html
    ///
    /// ```
    /// # use quantified::Quantified;
    /// let text: Quantified<String> = Quantified::Some("Hello, world!".to_string());
    /// // First, cast `Quantified<String>` to `Quantified<&String>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `text` on the stack.
    /// let text_length: Quantified<usize> = text.as_ref().map(|s| s.len());
    /// println!("still can print text: {:?}", text);
    /// ```
    pub const fn as_ref(&self) -> Quantified<&T> {
        match self {
            Self::None => Quantified::None,
            Self::Some(x) => Quantified::Some(&x),
            Self::Excluding(x) => Quantified::Excluding(&x),
            Self::All => Quantified::All,
        }
    }

    /// Converts from `&mut Quantified<T>` to `Quantified<&mut T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use quantified::Quantified;
    /// let mut x = Some(2);
    /// match x.as_mut() {
    ///     Some(v) => *v = 42,
    ///     None => {},
    /// }
    /// assert_eq!(x, Some(42));
    /// ```
    pub fn as_mut(&mut self) -> Quantified<&mut T> {
        match self {
            Self::None => Quantified::None,
            Self::Some(ts) => Quantified::Some(ts),
            Self::Excluding(ts) => Quantified::Excluding(ts),
            Self::All => Quantified::All,
        }
    }
}

impl<T: std::ops::Deref> Quantified<T> {
    /// Converts from `Quantified<T>` (or `&Quantified<T>`) to `Quantified<&T::Target>`.
    ///
    /// Leaves the original `Quantified` in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via [`Deref`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use quantified::Quantified;
    /// let x: Quantified<String> = Quantified::Some("hey".to_owned());
    /// assert_eq!(x.as_deref(), Quantified::Some("hey"));
    ///
    /// let x: Quantified<String> = Quantified::All;
    /// assert_eq!(x.as_deref(), Quantified::All);
    /// ```
    pub fn as_deref(&self) -> Quantified<&T::Target> {
        self.as_ref().map(|t| t.deref())
    }
}

impl<T: std::ops::DerefMut> Quantified<T> {
    /// Converts from `Quantified<T>` (or `&mut Quantified<T>`) to `Quantified<&mut T::Target>`.
    ///
    /// Leaves the original `Quantified` in-place, creating a new one containing a mutable
    /// reference to the inner type's `Deref::Target` type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use quantified::Quantified;
    /// let mut x: Quantified<String> = Quantified::Excluding("hey".to_owned());
    /// assert_eq!(x.as_deref_mut().map(|x| {
    ///     x.make_ascii_uppercase();
    ///     x
    /// }), Quantified::Excluding("HEY".to_owned().as_mut_str()));
    /// ```
    pub fn as_deref_mut(&mut self) -> Quantified<&mut T::Target> {
        self.as_mut().map(|t| t.deref_mut())
    }
}
