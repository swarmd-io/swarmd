use std::fmt::{Debug, Display};

use schemars::JsonSchema;
use serde::de::Visitor;
use serde::Serialize;
use slug::slugify;

#[derive(Clone, PartialEq, Eq, Serialize, Hash, JsonSchema, Default)]
#[serde(transparent)]
pub struct Slug(String);

const MAX_LEN: usize = 14;

impl<'de> serde::Deserialize<'de> for Slug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SlugVisitor;

        impl<'de> Visitor<'de> for SlugVisitor {
            type Value = Slug;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a slug")
            }

            fn visit_str<E>(self, value: &str) -> Result<Slug, E>
            where
                E: serde::de::Error,
            {
                let slugged = Slug::try_from(value);

                match slugged {
                    Ok(elt) => Ok(elt),
                    Err(err) => Err(serde::de::Error::custom(err)),
                }
            }
        }

        deserializer.deserialize_string(SlugVisitor)
    }
}

impl Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SlugDecodeError {
    #[error("`{0}` is not a slug")]
    NotASlug(String),
    #[error("`{0}` is too long, it must be < 14 char")]
    TooLong(String),
}
impl TryFrom<&str> for Slug {
    type Error = SlugDecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let slugged = slugify(value);
        if slugged == value {
            if slugged.len() >= MAX_LEN {
                Err(Self::Error::TooLong(value.to_string()))
            } else {
                Ok(Self(slugged))
            }
        } else {
            Err(Self::Error::NotASlug(value.to_string()))
        }
    }
}

impl TryFrom<String> for Slug {
    type Error = SlugDecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Slug {
    type Error = SlugDecodeError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.as_str())
    }
}

impl AsRef<String> for Slug {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
