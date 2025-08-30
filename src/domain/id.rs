use std::fmt;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use uuid::Uuid;


pub struct Id<T> {
    value: Uuid,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: Uuid) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct StringBranded<T> {
    value: String,
    _phantom: PhantomData<T>,
}

impl<T> fmt::Display for StringBranded<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Implémentation de Hash pour StringBranded
impl<T> Hash for StringBranded<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// Implémentation de PartialEq pour StringBranded
impl<T> PartialEq for StringBranded<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// Implémentation de Eq pour StringBranded
impl<T> Eq for StringBranded<T> {}



// Constructeur pour StringBranded
impl<T> StringBranded<T> {
    pub fn new(value: String) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}