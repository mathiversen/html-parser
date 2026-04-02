use html_escape::{decode_html_entities, encode_text};
use ownable::{IntoOwned, ToBorrowed, ToOwned};
#[cfg(feature = "test")]
use serde::Serialize;
use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Default, PartialEq, Eq, IntoOwned, ToBorrowed, ToOwned)]
#[cfg_attr(feature = "test", derive(Debug, Serialize))]
#[cfg_attr(feature = "test", serde(transparent))]
pub struct Attribute<'a>(pub Cow<'a, str>);

impl<'a> Attribute<'a> {
    pub fn as_str(&self) -> &str {
        self.0.borrow()
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Attribute<'static> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl<'a> Deref for Attribute<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<str> for Attribute<'a> {
    fn eq(&self, other: &str) -> bool {
        self.deref().eq(other)
    }
}

impl<'a> PartialEq<&str> for Attribute<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.deref().eq(*other)
    }
}

impl<'a> PartialEq<str> for &Attribute<'a> {
    fn eq(&self, other: &str) -> bool {
        (*self).deref().eq(other)
    }
}

impl Borrow<str> for Attribute<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

#[derive(Default, PartialEq, Eq, IntoOwned, ToBorrowed, ToOwned)]
#[cfg_attr(feature = "test", derive(Serialize))]
#[cfg_attr(feature = "test", serde(transparent))]
pub struct Text<'a>(pub Cow<'a, str>);

impl<'a> Text<'a> {
    pub fn as_str(&self) -> &str {
        self.0.borrow()
    }

    pub fn decode(&self) -> Cow<'_, str> {
        decode_html_entities(self.as_str())
    }

    pub fn encode(s: &'a str) -> Self {
        Self(encode_text(s))
    }
}

impl Debug for Text<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Text<'static> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl<'a> Deref for Text<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<str> for Text<'a> {
    fn eq(&self, other: &str) -> bool {
        self.deref().eq(other)
    }
}

impl<'a> PartialEq<&str> for Text<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.deref().eq(*other)
    }
}

impl<'a> PartialEq<str> for &Text<'a> {
    fn eq(&self, other: &str) -> bool {
        (*self).deref().eq(other)
    }
}
