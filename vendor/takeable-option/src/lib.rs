use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Takeable<T>(Option<T>);

impl<T> Takeable<T> {
    #[inline]
    pub fn new(value: T) -> Takeable<T> {
        Takeable(Some(value))
    }

    #[inline]
    pub fn new_empty() -> Takeable<T> {
        Takeable(None)
    }

    #[inline]
    pub fn take(takeable: &mut Takeable<T>) -> T {
        takeable.0.take().unwrap()
    }

    #[inline]
    pub fn try_take(takeable: &mut Takeable<T>) -> Option<T> {
        takeable.0.take()
    }

    #[inline]
    pub fn new_take(takeable: &mut Takeable<T>) -> Takeable<T> {
        Takeable::new(takeable.0.take().unwrap())
    }

    #[inline]
    pub fn insert(takeable: &mut Takeable<T>, new_takeable: T) -> Option<T> {
        let ret = takeable.0.take();
        takeable.0 = Some(new_takeable);
        ret
    }

    #[inline]
    pub fn to_opt(takeable: Takeable<T>) -> Option<T> {
        takeable.0
    }
}

impl<T> From<Option<T>> for Takeable<T> {
    #[inline]
    fn from(op: Option<T>) -> Self {
        Takeable(op)
    }
}

impl<T> Deref for Takeable<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<T> DerefMut for Takeable<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
