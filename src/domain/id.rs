use std::fmt;
use std::marker::PhantomData;
use uuid::Uuid;
use derive_more::Display;


pub struct Id<T> {
    value: Uuid,
    _phantom: PhantomData<T>,
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct StringBranded<T> {
    value: String,
    _phantom: PhantomData<T>,
}
impl<T> fmt::Display for StringBranded<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
