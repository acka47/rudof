use std::{result, str::FromStr};

use super::serde_string_or_struct::SerializeStringOrStruct;
use iri_s::IriSError;
use prefixmap::IriRef;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum IriRefOrWildcard {
    IriRef(IriRef),
    Wildcard {
        #[serde(rename = "type")]
        type_: String,
    },
}

impl FromStr for IriRefOrWildcard {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(IriRefOrWildcard::IriRef(iri_ref))
    }
}

impl SerializeStringOrStruct for IriRefOrWildcard {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            IriRefOrWildcard::IriRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
