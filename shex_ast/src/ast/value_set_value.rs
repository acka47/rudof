use crate::{ast::serde_string_or_struct::*, Deref, DerefError, LangOrWildcard};
use crate::{Exclusion, IriExclusion, LanguageExclusion, LiteralExclusion, NumericLiteral};
use iri_s::IriSError;
use rust_decimal::Decimal;
use serde::ser::SerializeMap;
use serde::{
    de::{self, MapAccess, Unexpected, Visitor},
    Deserialize, Serialize, Serializer,
};
use serde_derive::{Deserialize, Serialize};
use srdf::lang::Lang;
use std::{fmt, result, str::FromStr};

use super::{
    iri_ref::IriRef, iri_ref_or_wildcard::IriRefOrWildcard, string_or_wildcard::StringOrWildcard,
    ObjectValue,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ValueSetValue {
    IriStem {
        stem: IriRef,
    },
    IriStemRange {
        stem: IriRefOrWildcard,
        exclusions: Option<Vec<IriExclusion>>,
    },
    LiteralStem {
        stem: String,
    },
    LiteralStemRange {
        stem: StringOrWildcard,
        exclusions: Option<Vec<LiteralExclusion>>,
    },
    Language {
        language_tag: Lang,
    },
    LanguageStem {
        stem: Lang,
    },
    LanguageStemRange {
        stem: LangOrWildcard,
        exclusions: Option<Vec<LanguageExclusion>>,
    },

    ObjectValue(ObjectValue),
}

impl ValueSetValue {
    pub fn iri(iri: IriRef) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValue::IriRef(iri))
    }

    pub fn literal(value: &str, language: Option<Lang>, type_: Option<IriRef>) -> ValueSetValue {
        let ov = ObjectValue::ObjectLiteral {
            value: value.to_string(),
            language,
            type_,
        };
        ValueSetValue::ObjectValue(ov)
    }

    pub fn object_value(value: ObjectValue) -> ValueSetValue {
        ValueSetValue::ObjectValue(value)
    }

    pub fn language(lang: Lang) -> ValueSetValue {
        ValueSetValue::Language { language_tag: lang }
    }

    pub fn language_stem(lang: Lang) -> ValueSetValue {
        ValueSetValue::LanguageStem { stem: lang }
    }

    pub fn literal_stem(stem: String) -> ValueSetValue {
        ValueSetValue::LiteralStem { stem }
    }
}

impl Deref for ValueSetValue {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        match self {
            ValueSetValue::ObjectValue(ov) => {
                let ov = ov.deref(base, prefixmap)?;
                Ok(ValueSetValue::ObjectValue(ov))
            }
            ValueSetValue::Language { language_tag } => Ok(ValueSetValue::Language {
                language_tag: language_tag.clone(),
            }),
            ValueSetValue::LanguageStem { stem } => {
                Ok(ValueSetValue::LanguageStem { stem: stem.clone() })
            }
            ValueSetValue::IriStem { stem } => Ok(ValueSetValue::IriStem { stem: stem.clone() }),
            ValueSetValue::IriStemRange { stem, exclusions } => Ok(ValueSetValue::IriStemRange {
                stem: stem.clone(),
                exclusions: exclusions.clone(),
            }),
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                Ok(ValueSetValue::LanguageStemRange {
                    stem: stem.clone(),
                    exclusions: exclusions.clone(),
                })
            }
            ValueSetValue::LiteralStem { stem } => {
                Ok(ValueSetValue::LiteralStem { stem: stem.clone() })
            }
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                Ok(ValueSetValue::LiteralStemRange {
                    stem: stem.clone(),
                    exclusions: exclusions.clone(),
                })
            }
        }
    }
}

impl SerializeStringOrStruct for ValueSetValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ValueSetValue::ObjectValue(ObjectValue::IriRef(ref r)) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl FromStr for ValueSetValue {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(ValueSetValue::ObjectValue(ObjectValue::IriRef(iri_ref)))
    }
}

#[derive(Debug, PartialEq)]
enum ValueSetValueType {
    IriStem,
    LanguageStem,
    LiteralStem,
    IriStemRange,
    LanguageStemRange,
    LiteralStemRange,
    Language,
    Boolean,
    Integer,
    Decimal,
    Double,
    Other(IriRef),
}

impl ValueSetValueType {
    fn parse(s: &str) -> Result<ValueSetValueType, IriSError> {
        match s {
            "IriStem" => Ok(ValueSetValueType::IriStem),
            "LanguageStem" => Ok(ValueSetValueType::LanguageStem),
            "LiteralStem" => Ok(ValueSetValueType::LiteralStem),
            "Language" => Ok(ValueSetValueType::Language),
            "IriStemRange" => Ok(ValueSetValueType::IriStemRange),
            "LanguageStemRange" => Ok(ValueSetValueType::LanguageStemRange),
            "LiteralStemRange" => Ok(ValueSetValueType::LiteralStemRange),
            BOOLEAN_STR => Ok(ValueSetValueType::Boolean),
            DECIMAL_STR => Ok(ValueSetValueType::Decimal),
            DOUBLE_STR => Ok(ValueSetValueType::Double),
            INTEGER_STR => Ok(ValueSetValueType::Integer),
            other => {
                let iri = FromStr::from_str(other)?;
                Ok(ValueSetValueType::Other(iri))
            }
        }
    }
}

const BOOLEAN_STR: &str = "http://www.w3.org/2001/XMLSchema#boolean";
const INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#integer";
const DOUBLE_STR: &str = "http://www.w3.org/2001/XMLSchema#double";
const DECIMAL_STR: &str = "http://www.w3.org/2001/XMLSchema#decimal";

impl Serialize for ValueSetValue {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueSetValue::ObjectValue(v) => match v {
                ObjectValue::BooleanLiteral { value } => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", BOOLEAN_STR)?;
                    let value_str = if *value { "true" } else { "false" };
                    map.serialize_entry("value", value_str)?;
                    map.end()
                }
                ObjectValue::NumericLiteral(num) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", get_type_str(num))?;
                    map.serialize_entry("value", &num.to_string())?;
                    map.end()
                }
                ObjectValue::IriRef(iri) => serializer.serialize_str(iri.to_string().as_str()),
                ObjectValue::ObjectLiteral {
                    value,
                    language,
                    type_,
                } => {
                    let mut map = serializer.serialize_map(Some(3))?;
                    match type_ {
                        Some(t) => map.serialize_entry("type", t.to_string().as_str())?,
                        None => {}
                    };
                    match language {
                        Some(lan) => map.serialize_entry("language", lan.value().as_str())?,
                        None => {}
                    }
                    map.serialize_entry("value", value)?;
                    map.end()
                }
            },
            ValueSetValue::Language { language_tag } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Language")?;
                map.serialize_entry("languageTag", &language_tag.value())?;
                map.end()
            }
            ValueSetValue::IriStem { stem } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IriStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            ValueSetValue::IriStemRange { stem, exclusions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IriStemRange")?;
                map.serialize_entry("stem", stem)?;
                map.serialize_entry("exclusions", exclusions)?;
                map.end()
            }
            ValueSetValue::LanguageStem { stem } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStemRange")?;
                map.serialize_entry("stem", stem)?;
                map.serialize_entry("exclusions", exclusions)?;
                map.end()
            }
            ValueSetValue::LiteralStem { stem } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStemRange")?;
                map.serialize_entry("stem", stem)?;
                map.serialize_entry("exclusions", exclusions)?;
                map.end()
            }
        }
    }
}

fn get_type_str(n: &NumericLiteral) -> &str {
    match n {
        NumericLiteral::Integer(_) => INTEGER_STR,
        NumericLiteral::Double(_) => DOUBLE_STR,
        NumericLiteral::Decimal(_) => DECIMAL_STR,
    }
}

#[derive(Serialize, Debug)]
enum Stem {
    Str(String),
    Wildcard {
        #[serde(rename = "type")]
        type_: String,
    },
}

#[derive(Debug)]
struct NoStringOrWildCard;

#[derive(Debug)]
struct NoString;

#[derive(Debug)]
struct NoLanguage;

#[derive(Debug)]
enum ErrStemIriRef {
    StemIsWildcard,
    IriError { err: IriSError },
}

impl Stem {
    fn as_iri(&self) -> Result<IriRef, ErrStemIriRef> {
        match self {
            Stem::Str(s) => {
                let iri_ref =
                    IriRef::from_str(s.as_str()).map_err(|e| ErrStemIriRef::IriError { err: e })?;
                Ok(iri_ref)
            }
            _ => Err(ErrStemIriRef::StemIsWildcard),
        }
    }

    fn as_language(&self) -> Result<String, NoLanguage> {
        match self {
            Stem::Str(s) => Ok(s.clone()),
            _ => Err(NoLanguage),
        }
    }

    fn as_string(&self) -> Result<String, NoString> {
        match self {
            Stem::Str(s) => Ok(s.clone()),
            _ => Err(NoString),
        }
    }
    fn as_string_or_wildcard(&self) -> Result<StringOrWildcard, NoStringOrWildCard> {
        match self {
            Stem::Str(s) => Ok(StringOrWildcard::String(s.clone())),
            Stem::Wildcard { type_ } => Ok(StringOrWildcard::Wildcard {
                type_: type_.clone(),
            }),
        }
    }
}

impl<'de> Deserialize<'de> for Stem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("stem value or wildcard with `type`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct StemVisitor;

        const FIELDS: &'static [&'static str] = &["type"];

        impl<'de> Visitor<'de> for StemVisitor {
            type Value = Stem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("stem value")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                /*FromStr::from_str(s)
                .map_err(|e| de::Error::custom(format!("Error parsing string `{s}`: {e}"))) */
                todo!()
            }

            fn visit_map<V>(self, mut map: V) -> Result<Stem, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<StemType> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ = StemType::parse(&value.as_str()).map_err(|e| {
                                de::Error::custom(format!(
                                    "Error parsing stem type, found: {value}. Error: {e}"
                                ))
                            })?;
                            type_ = Some(parsed_type_);
                        }
                    }
                }
                match type_ {
                    Some(StemType::Wildcard) => todo!(),
                    _ => todo!(),
                }
            }
        }
        deserializer.deserialize_any(StemVisitor)
    }
}

enum StemType {
    Str,
    Wildcard,
}

impl StemType {
    fn parse(s: &str) -> Result<StemType, IriSError> {
        todo!()
    }
}

impl<'de> Deserialize<'de> for ValueSetValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            Value,
            Language,
            Stem,
            Exclusions,
            LanguageTag,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(
                                "field of value set value: `type` or `value` or `language` or `stem` or `exclusions`",
                            )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "value" => Ok(Field::Value),
                            "stem" => Ok(Field::Stem),
                            "language" => Ok(Field::Language),
                            "languageTag" => Ok(Field::LanguageTag),
                            "exclusions" => Ok(Field::Exclusions),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ValueSetValueVisitor;

        const FIELDS: &'static [&'static str] = &[
            "type",
            "value",
            "stem",
            "language",
            "languageTag",
            "exclusions",
        ];

        impl<'de> Visitor<'de> for ValueSetValueVisitor {
            type Value = ValueSetValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ValueSet value")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(s)
                    .map_err(|e| de::Error::custom(format!("Error parsing string `{s}`: {e}")))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ValueSetValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<ValueSetValueType> = None;
                let mut stem: Option<Stem> = None;
                let mut value: Option<String> = None;
                let mut language_tag: Option<String> = None;
                let mut language: Option<String> = None;
                let mut exclusions: Option<Vec<Exclusion>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ =
                                ValueSetValueType::parse(&value.as_str()).map_err(|e| {
                                    de::Error::custom(format!(
                                    "Error parsing ValueSetValue type, found: {value}. Error: {e}"
                                ))
                                })?;
                            type_ = Some(parsed_type_);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Language => {
                            if language.is_some() {
                                return Err(de::Error::duplicate_field("language"));
                            }
                            language = Some(map.next_value()?);
                        }
                        Field::Stem => {
                            if stem.is_some() {
                                return Err(de::Error::duplicate_field("stem"));
                            }
                            stem = Some(map.next_value()?);
                        }
                        Field::Exclusions => {
                            if exclusions.is_some() {
                                return Err(de::Error::duplicate_field("exclusions"));
                            }
                            exclusions = Some(map.next_value()?);
                        }
                        Field::LanguageTag => {
                            if language_tag.is_some() {
                                return Err(de::Error::duplicate_field("languageTag"));
                            }
                            language_tag = Some(map.next_value()?);
                        }
                    }
                }
                match type_ {
                    Some(ValueSetValueType::LiteralStemRange) => match stem {
                        Some(stem) => match exclusions {
                            Some(excs) => {
                                let lit_excs = Exclusion::parse_literal_exclusions(excs).map_err(|e| {
                                    de::Error::custom("LiteralStemRange: some exclusions are not literal exclusions: {e:?}")
                                })?;
                                let stem = stem.as_string_or_wildcard().map_err(|e| {
                                    de::Error::custom(format!("LiteralStemRange: stem is not string or wildcard. stem `{stem:?}`: {e:?}"))
                                })?;
                                Ok(ValueSetValue::LiteralStemRange {
                                    stem: stem,
                                    exclusions: Some(lit_excs),
                                })
                            }
                            None => {
                                todo!()
                            }
                        },
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::LanguageStemRange) => {
                        todo!()
                    }
                    Some(ValueSetValueType::IriStemRange) => {
                        todo!()
                    }
                    Some(ValueSetValueType::LiteralStem) => match stem {
                        Some(stem) => {
                            let stem = stem.as_string().map_err(|e| {
                                de::Error::custom("LiteralStem: value of stem must be a string")
                            })?;
                            Ok(ValueSetValue::LiteralStem { stem: stem })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::LanguageStem) => match stem {
                        Some(stem) => {
                            let stem = stem.as_language().map_err(|e| {
                                de::Error::custom("LanguageStem: stem is not a language")
                            })?;
                            Ok(ValueSetValue::LanguageStem {
                                stem: Lang::new(&stem),
                            })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::Language) => match language_tag {
                        Some(language_tag) => Ok(ValueSetValue::Language {
                            language_tag: Lang::new(language_tag.as_str()),
                        }),
                        None => Err(de::Error::missing_field("languageTag")),
                    },
                    Some(ValueSetValueType::IriStem) => match stem {
                        Some(stem) => {
                            let iri_ref = stem.as_iri().map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse stem `{stem:?}` as IRIREF for IriStem. Error: {e:?}"
                                ))
                            })?;
                            Ok(ValueSetValue::IriStem { stem: iri_ref })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::Boolean) => match value {
                        Some(s) => match s.as_str() {
                            "false" => {
                                Ok(ValueSetValue::ObjectValue(ObjectValue::BooleanLiteral {
                                    value: false,
                                }))
                            }
                            "true" => Ok(ValueSetValue::ObjectValue(ObjectValue::BooleanLiteral {
                                value: true,
                            })),
                            _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &self)),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Double) => match value {
                        Some(s) => {
                            let n = f64::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as double: Error {e}"
                                ))
                            })?;
                            Ok(ValueSetValue::ObjectValue(ObjectValue::NumericLiteral(
                                NumericLiteral::double(n),
                            )))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Decimal) => match value {
                        Some(s) => {
                            let n = Decimal::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as decimal: Error {e}"
                                ))
                            })?;
                            Ok(ValueSetValue::ObjectValue(ObjectValue::NumericLiteral(
                                NumericLiteral::decimal(n),
                            )))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Integer) => match value {
                        Some(s) => {
                            let n = isize::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as integer: Error {e}"
                                ))
                            })?;
                            Ok(ValueSetValue::ObjectValue(ObjectValue::NumericLiteral(
                                NumericLiteral::integer(n),
                            )))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Other(iri)) => match value {
                        Some(v) => match language_tag {
                            Some(lang) => {
                                Ok(ValueSetValue::ObjectValue(ObjectValue::ObjectLiteral {
                                    value: v,
                                    language: Some(Lang::new(&lang)),
                                    type_: Some(iri),
                                }))
                            }
                            None => Ok(ValueSetValue::ObjectValue(ObjectValue::ObjectLiteral {
                                value: v,
                                language: None,
                                type_: Some(iri),
                            })),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    None => match value {
                        Some(value) => match language {
                            Some(language) => {
                                Ok(ValueSetValue::ObjectValue(ObjectValue::ObjectLiteral {
                                    value,
                                    language: Some(Lang::new(&language)),
                                    type_: None,
                                }))
                            }
                            None => Ok(ValueSetValue::ObjectValue(ObjectValue::ObjectLiteral {
                                value,
                                language: None,
                                type_: None,
                            })),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                }
            }
        }

        deserializer.deserialize_any(ValueSetValueVisitor)
    }
}
