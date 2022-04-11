extern crate bson;
extern crate flexbuffers;
#[cfg(feature = "http")]
extern crate http;
extern crate json5;
extern crate postcard;
extern crate rmp_serde;
extern crate ron;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_lexpr;
extern crate serde_pickle as pickle;
extern crate serde_qs;
extern crate serde_xml_rs;
extern crate serde_yaml;

use core::str::from_utf8;

pub mod prelude {
    pub extern crate bson;
    pub extern crate flexbuffers;
    pub extern crate json5;
    pub extern crate postcard;
    pub extern crate rmp_serde as messagepack;
    pub extern crate ron;
    pub extern crate serde_cbor as cbor;
    pub extern crate serde_derive;
    pub extern crate serde_json as json;
    pub extern crate serde_lexpr as lexpr;
    pub extern crate serde_pickle as pickle;
    pub extern crate serde_qs as url;
    pub extern crate serde_xml_rs as xml;
    pub extern crate serde_yaml as yaml;
}

#[cfg(feature = "http")]
use crate::Error::InvalidHeaderValue;
#[cfg(feature = "http")]
use actix_http::header::TryIntoHeaderValue;
use derive_more::Display;
#[cfg(feature = "http")]
use http::{header::ToStrError, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_xml_rs::Error::FromUtf8Error;
use std::convert::{Infallible, TryFrom, TryInto};
use std::ops::{Deref, DerefMut};
use std::str::Utf8Error;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Debug)]
pub enum ContentType {
    Bson,
    Cbor,
    FlexBuffers,
    Json,
    Json5,
    Lexpr,
    MessagePack,
    Pickle,
    Postcard,
    Ron,
    Toml,
    Url,
    Yaml,
    Xml,
}

impl TryFrom<&str> for ContentType {
    type Error = crate::Error;

    fn try_from(s: &str) -> std::result::Result<ContentType, Self::Error> {
        match s.to_lowercase().as_str() {
            "bson" => Ok(ContentType::Bson),
            "application/bson" => Ok(ContentType::Bson),
            "application/x-bson" => Ok(ContentType::Bson),
            "cbor" => Ok(ContentType::Cbor),
            "application/cbor" => Ok(ContentType::Cbor),
            "application/x-cbor" => Ok(ContentType::Cbor),
            "flexbuffers" => Ok(ContentType::FlexBuffers),
            "application/flexbuffers" => Ok(ContentType::FlexBuffers),
            "application/x-flexbuffers" => Ok(ContentType::FlexBuffers),
            "json" => Ok(ContentType::Json),
            "application/json" => Ok(ContentType::Json),
            "application/x-json" => Ok(ContentType::Json),
            "json5" => Ok(ContentType::Json5),
            "application/json5" => Ok(ContentType::Json5),
            "application/x-json5" => Ok(ContentType::Json5),
            "lexpr" => Ok(ContentType::Lexpr),
            "application/lexpr" => Ok(ContentType::Lexpr),
            "application/x-lexpr" => Ok(ContentType::Lexpr),
            "messagepack" => Ok(ContentType::MessagePack),
            "application/messagepack" => Ok(ContentType::MessagePack),
            "application/x-messagepack" => Ok(ContentType::MessagePack),
            "pickle" => Ok(ContentType::Pickle),
            "application/pickle" => Ok(ContentType::Pickle),
            "application/x-pickle" => Ok(ContentType::Pickle),
            "postcard" => Ok(ContentType::Postcard),
            "application/postcard" => Ok(ContentType::Postcard),
            "application/x-postcard" => Ok(ContentType::Postcard),
            "ron" => Ok(ContentType::Ron),
            "application/ron" => Ok(ContentType::Ron),
            "application/x-ron" => Ok(ContentType::Ron),
            "toml" => Ok(ContentType::Toml),
            "application/toml" => Ok(ContentType::Toml),
            "application/x-toml" => Ok(ContentType::Toml),
            "url" => Ok(ContentType::Url),
            "application/url" => Ok(ContentType::Url),
            "application/x-url" => Ok(ContentType::Url),
            "yaml" => Ok(ContentType::Yaml),
            "application/yaml" => Ok(ContentType::Yaml),
            "application/x-yaml" => Ok(ContentType::Yaml),
            "xml" => Ok(ContentType::Xml),
            "application/xml" => Ok(ContentType::Xml),
            "application/x-xml" => Ok(ContentType::Xml),
            _ => Err(Error::UnknownContentTypeMatchFromStr(s.to_string())),
        }
    }
}

impl TryFrom<&ContentType> for ContentType {
    type Error = crate::Error;

    fn try_from(h: &ContentType) -> std::result::Result<ContentType, Self::Error> {
        match h {
            Self::Bson => Ok(Self::Bson),
            Self::Cbor => Ok(Self::Cbor),
            Self::FlexBuffers => Ok(Self::FlexBuffers),
            Self::Json => Ok(Self::Json),
            Self::Json5 => Ok(Self::Json5),
            Self::Lexpr => Ok(Self::Lexpr),
            Self::MessagePack => Ok(Self::MessagePack),
            Self::Pickle => Ok(Self::Pickle),
            Self::Postcard => Ok(Self::Postcard),
            Self::Ron => Ok(Self::Ron),
            Self::Toml => Ok(Self::Toml),
            Self::Url => Ok(Self::Url),
            Self::Yaml => Ok(Self::Yaml),
            Self::Xml => Ok(Self::Xml),
        }
    }
}

#[cfg(feature = "http")]
impl TryIntoHeaderValue for ContentType {
    type Error = http::header::InvalidHeaderValue;

    fn try_into_value(self) -> std::result::Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(match self {
            ContentType::Bson => "application/x-bson",
            ContentType::Cbor => "application/x-cbor",
            ContentType::FlexBuffers => "application/x-flexbuffers",
            ContentType::Json => "application/json",
            ContentType::Json5 => "application/json5",
            ContentType::Lexpr => "application/x-lexpr",
            ContentType::MessagePack => "application/x-messagepack",
            ContentType::Pickle => "application/x-pickle",
            ContentType::Postcard => "application/x-postcard",
            ContentType::Ron => "application/ron",
            ContentType::Toml => "application/toml",
            ContentType::Url => "application/x-url",
            ContentType::Yaml => "application/yaml",
            ContentType::Xml => "application/xml",
        })
    }
}
#[cfg(feature = "http")]
impl TryIntoHeaderValue for &ContentType {
    type Error = http::header::InvalidHeaderValue;

    fn try_into_value(self) -> std::result::Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(match self {
            ContentType::Bson => "application/x-bson",
            ContentType::Cbor => "application/x-cbor",
            ContentType::FlexBuffers => "application/x-flexbuffers",
            ContentType::Json => "application/json",
            ContentType::Json5 => "application/json5",
            ContentType::Lexpr => "application/x-lexpr",
            ContentType::MessagePack => "application/x-messagepack",
            ContentType::Pickle => "application/x-pickle",
            ContentType::Postcard => "application/x-postcard",
            ContentType::Ron => "application/ron",
            ContentType::Toml => "application/toml",
            ContentType::Url => "application/x-url",
            ContentType::Yaml => "application/yaml",
            ContentType::Xml => "application/xml",
        })
    }
}

#[cfg(feature = "http")]
impl TryFrom<HeaderValue> for ContentType {
    type Error = Error;

    fn try_from(h: HeaderValue) -> std::result::Result<ContentType, Self::Error> {
        h.to_str()
            .map_err(Error::from)
            .and_then(ContentType::try_from)
    }
}

#[cfg(feature = "http")]
impl TryFrom<&HeaderValue> for ContentType {
    type Error = Error;

    fn try_from(h: &HeaderValue) -> std::result::Result<ContentType, Self::Error> {
        h.to_str()
            .map_err(Error::from)
            .and_then(ContentType::try_from)
    }
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Infallible - This error should have been infallible")]
    Infallible,
    #[display(fmt = "Converting Raw Data to UTF8 failed: {}", _0)]
    ByteToUTF8ConversionFailure(Utf8Error),
    #[display(fmt = "Unknown content type match from str: {}", _0)]
    UnknownContentTypeMatchFromStr(String),
    #[display(fmt = "BSON encoder/decoder error: {}", _0)]
    BsonSerializationFailure(bson::ser::Error),
    #[display(fmt = "BSON encode/decoder error: {}", _0)]
    BsonDeserializationFailure(bson::de::Error),
    #[display(fmt = "CBOR encoder/decoder error: {}", _0)]
    CborFailure(serde_cbor::Error),
    #[display(fmt = "Flexbuffers encoder/decoder error: {}", _0)]
    FlexBuffersSerializationFailure(flexbuffers::SerializationError),
    #[display(fmt = "Flexbuffers encoder/decoder error: {}", _0)]
    FlexBuffersDeserializationFailure(flexbuffers::DeserializationError),
    #[display(fmt = "JSON encoder/decoder error: {}", _0)]
    JsonError(serde_json::Error),
    #[display(fmt = "JSON5 encoder/decoder error: {}", _0)]
    Json5Error(json5::Error),
    #[display(fmt = "LEXPR encoder/decoder error: {}", _0)]
    LexprError(serde_lexpr::Error),
    #[display(fmt = "MessagePack encoder/decoder error: {}", _0)]
    MessagePackEncodeError(rmp_serde::encode::Error),
    #[display(fmt = "MessagePack encoder/decoder error: {}", _0)]
    MessagePackDecodeError(rmp_serde::decode::Error),
    #[display(fmt = "Pickle encoder/decoder error: {}", _0)]
    PickleError(serde_pickle::Error),
    #[display(fmt = "Postcard encoder/decoder error: {}", _0)]
    PostcardError(postcard::Error),
    #[display(fmt = "RON encoder/decoder error: {}", _0)]
    RonError(ron::Error),
    #[display(fmt = "TOML encoder/decoder error: {}", _0)]
    TomlSerializationFailure(toml::ser::Error),
    #[display(fmt = "TOML encoder/decoder error: {}", _0)]
    TomlDeserializationFailure(toml::de::Error),
    #[display(fmt = "URL encoder/decoder error: {}", _0)]
    UrlEncodingFailure(serde_qs::Error),
    #[display(fmt = "YAML encoder/decoder error: {}", _0)]
    YamlError(serde_yaml::Error),
    #[display(fmt = "XML encoder/decoder error: {}", _0)]
    XmlError(serde_xml_rs::Error),
    #[display(fmt = "Type is not supported for encoding/decoding: {:?}", _0)]
    TypeDoesNotSupportSerialization(ContentType),
    #[cfg(feature = "http")]
    #[display(fmt = "Failed to convert `HeaderValue` to a ContentType: {}", _0)]
    FailedConvertingHeaderValueToContentType(http::header::ToStrError),
    #[cfg(feature = "http")]
    #[display(fmt = "Invalid Header Value found: {}", _0)]
    InvalidHeaderValue(http::header::InvalidHeaderValue),
}

#[cfg(feature = "http")]
impl From<http::header::InvalidHeaderValue> for Error {
    fn from(e: http::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}

#[cfg(feature = "http")]
impl From<http::header::ToStrError> for Error {
    fn from(e: ToStrError) -> Self {
        Self::FailedConvertingHeaderValueToContentType(e)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::ByteToUTF8ConversionFailure(e)
    }
}
impl From<bson::ser::Error> for Error {
    fn from(e: bson::ser::Error) -> Self {
        Error::BsonSerializationFailure(e)
    }
}
impl From<bson::de::Error> for Error {
    fn from(e: bson::de::Error) -> Self {
        Error::BsonDeserializationFailure(e)
    }
}
impl From<serde_cbor::Error> for Error {
    fn from(e: serde_cbor::Error) -> Self {
        Error::CborFailure(e)
    }
}
impl From<flexbuffers::SerializationError> for Error {
    fn from(e: flexbuffers::SerializationError) -> Self {
        Error::FlexBuffersSerializationFailure(e)
    }
}
impl From<flexbuffers::DeserializationError> for Error {
    fn from(e: flexbuffers::DeserializationError) -> Self {
        Error::FlexBuffersDeserializationFailure(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}
impl From<json5::Error> for Error {
    fn from(e: json5::Error) -> Self {
        Error::Json5Error(e)
    }
}
impl From<serde_lexpr::Error> for Error {
    fn from(e: serde_lexpr::Error) -> Self {
        Error::LexprError(e)
    }
}
impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Error::MessagePackEncodeError(e)
    }
}
impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Error::MessagePackDecodeError(e)
    }
}
impl From<serde_pickle::Error> for Error {
    fn from(e: serde_pickle::Error) -> Self {
        Error::PickleError(e)
    }
}
impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        Error::PostcardError(e)
    }
}
impl From<ron::Error> for Error {
    fn from(e: ron::Error) -> Self {
        Error::RonError(e)
    }
}
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::TomlSerializationFailure(e)
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlDeserializationFailure(e)
    }
}
impl From<serde_qs::Error> for Error {
    fn from(e: serde_qs::Error) -> Self {
        Error::UrlEncodingFailure(e)
    }
}
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlError(e)
    }
}
impl From<serde_xml_rs::Error> for Error {
    fn from(e: serde_xml_rs::Error) -> Self {
        Error::XmlError(e)
    }
}

pub trait TryToString {
    type Error;
    fn try_to_string(&self) -> std::result::Result<String, Self::Error>;
}

pub trait SimpleEncoder
where
    Self: serde::Serialize,
{
    fn encode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<Encoded>;
}

impl<T> SimpleEncoder for T
where
    T: Serialize,
{
    fn encode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<Encoded> {
        use std::str::from_utf8;
        let bson = |o: &T| -> Result<Encoded> { bson::to_vec(o).try_into() };
        let cbor = |o: &T| -> Result<Encoded> { serde_cbor::to_vec(o).try_into() };
        let flexbuffers = |o: &T| -> Result<Encoded> { flexbuffers::to_vec(o).try_into() };
        let json = |o: &T| -> Result<Encoded> { serde_json::to_vec(o).try_into() };
        let json5 = |o: &T| -> Result<Encoded> { json5::to_string(o).try_into() };
        let lexpr = |o: &T| -> Result<Encoded> { serde_lexpr::to_vec(o).try_into() };
        let message_pack = |o: &T| -> Result<Encoded> { rmp_serde::to_vec(o).try_into() };
        let pickle =
            |o: &T| -> Result<Encoded> { serde_pickle::to_vec(o, Default::default()).try_into() };
        let postcard = |o: &T| -> Result<Encoded> { postcard::to_allocvec(o).try_into() };
        let ron = |o: &T| -> Result<Encoded> { ron::to_string(o).try_into() };
        let toml = |o: &T| -> Result<Encoded> { toml::to_vec(o).try_into() };
        let url = |o: &T| -> Result<Encoded> { serde_qs::to_string(o).try_into() };
        let yaml = |o: &T| -> Result<Encoded> { serde_yaml::to_vec(o).try_into() };
        let xml = |o: &T| -> Result<Encoded> { serde_xml_rs::to_string(o).try_into() };
        match content_type.try_into().map_err(|e| e.into())? {
            ContentType::Bson => bson(self),
            ContentType::Cbor => cbor(self),
            ContentType::FlexBuffers => flexbuffers(self),
            ContentType::Json => json(self),
            ContentType::Json5 => json5(self),
            ContentType::Lexpr => lexpr(self),
            ContentType::MessagePack => message_pack(self),
            ContentType::Pickle => pickle(self),
            ContentType::Postcard => postcard(self),
            ContentType::Ron => ron(self),
            ContentType::Toml => toml(self),
            ContentType::Url => url(self),
            ContentType::Yaml => yaml(self),
            ContentType::Xml => xml(self),
        }
    }
}

pub trait SimpleDecoder<T> {
    fn decode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<T>;
}

impl<T> SimpleDecoder<Decoded<T>> for &[u8]
where
    T: DeserializeOwned,
{
    fn decode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<Decoded<T>> {
        let bson = |o: &[u8]| -> Result<Decoded<T>> { bson::from_slice(o).try_into() };
        let cbor = |o: &[u8]| -> Result<Decoded<T>> { serde_cbor::from_slice(o).try_into() };
        let flexbuffers =
            |o: &[u8]| -> Result<Decoded<T>> { flexbuffers::from_slice(o).try_into() };
        let json = |o: &[u8]| -> Result<Decoded<T>> { serde_json::from_slice(o).try_into() };
        let json5 = |o: &[u8]| -> Result<Decoded<T>> {
            std::str::from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| json5::from_str(str).try_into())
        };
        let lexpr = |o: &[u8]| -> Result<Decoded<T>> { serde_lexpr::from_slice(o).try_into() };
        let message_pack = |o: &[u8]| -> Result<Decoded<T>> { rmp_serde::from_slice(o).try_into() };
        let pickle = |o: &[u8]| -> Result<Decoded<T>> {
            serde_pickle::from_slice(o, Default::default()).try_into()
        };
        let postcard = |o: &[u8]| -> Result<Decoded<T>> { postcard::from_bytes(o).try_into() };
        let ron = |o: &[u8]| -> Result<Decoded<T>> {
            std::str::from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| ron::from_str(str).try_into())
        };
        let toml = |o: &[u8]| -> Result<Decoded<T>> { toml::from_slice(o).try_into() };
        let url = |o: &[u8]| -> Result<Decoded<T>> { serde_qs::from_bytes(o).try_into() };
        let yaml = |o: &[u8]| -> Result<Decoded<T>> { serde_yaml::from_slice(o).try_into() };
        let xml = |o: &[u8]| -> Result<Decoded<T>> {
            std::str::from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| serde_xml_rs::from_str(str).try_into())
        };
        match content_type.try_into().map_err(|e| e.into())? {
            ContentType::Bson => bson(self),
            ContentType::Cbor => cbor(self),
            ContentType::FlexBuffers => flexbuffers(self),
            ContentType::Json => json(self),
            ContentType::Json5 => json5(self),
            ContentType::Lexpr => lexpr(self),
            ContentType::MessagePack => message_pack(self),
            ContentType::Pickle => pickle(self),
            ContentType::Postcard => postcard(self),
            ContentType::Ron => ron(self),
            ContentType::Toml => toml(self),
            ContentType::Url => url(self),
            ContentType::Yaml => yaml(self),
            ContentType::Xml => xml(self),
        }
    }
}

pub struct Encoded {
    inner: Vec<u8>,
}

impl PartialEq<Self> for Encoded {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<String> for Encoded {
    fn eq(&self, other: &String) -> bool {
        if let Ok(self_string) = self.try_to_string() {
            self_string == *other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for Encoded {
    fn eq(&self, other: &&str) -> bool {
        if let Ok(self_string) = self.try_to_string() {
            self_string == *other
        } else {
            false
        }
    }
}

impl Eq for Encoded {}

impl From<Vec<u8>> for Encoded {
    fn from(v: Vec<u8>) -> Self {
        Encoded { inner: v }
    }
}

impl From<String> for Encoded {
    fn from(s: String) -> Self {
        Encoded {
            inner: s.as_bytes().to_vec(),
        }
    }
}

impl From<&str> for Encoded {
    fn from(s: &str) -> Self {
        Encoded {
            inner: s.as_bytes().to_vec(),
        }
    }
}

impl Deref for Encoded {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Encoded {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<E> TryFrom<std::result::Result<Vec<u8>, E>> for Encoded
where
    E: Into<Error>,
{
    type Error = Error;
    fn try_from(value: std::result::Result<Vec<u8>, E>) -> std::result::Result<Self, Self::Error> {
        value.map(Encoded::from).map_err(|e| e.into())
    }
}

impl<E> TryFrom<std::result::Result<String, E>> for Encoded
where
    E: Into<Error>,
{
    type Error = Error;
    fn try_from(value: std::result::Result<String, E>) -> std::result::Result<Self, Self::Error> {
        value.map(Encoded::from).map_err(|e| e.into())
    }
}

impl<E> TryFrom<std::result::Result<&str, E>> for Encoded
where
    E: Into<Error>,
{
    type Error = Error;
    fn try_from(value: std::result::Result<&str, E>) -> std::result::Result<Self, Self::Error> {
        value.map(Encoded::from).map_err(|e| e.into())
    }
}

impl TryToString for Encoded {
    type Error = Error;
    fn try_to_string(&self) -> std::result::Result<String, Self::Error> {
        from_utf8(self).map_err(Error::from).map(|s| s.to_string())
    }
}

pub struct Decoded<T>
where
    T: DeserializeOwned,
{
    pub(crate) inner: T,
}

pub trait DecodedInto<T> {
    fn into(self) -> T;
}

impl<T, E> TryFrom<std::result::Result<T, E>> for Decoded<T>
where
    T: DeserializeOwned,
    E: Into<Error>,
{
    type Error = Error;

    fn try_from(res: std::result::Result<T, E>) -> std::result::Result<Self, Self::Error> {
        res.map_err(|e| e.into()).map(Decoded::from)
    }
}

impl<T: DeserializeOwned> From<T> for Decoded<T> {
    fn from(t: T) -> Self {
        Decoded { inner: t }
    }
}

impl<T: DeserializeOwned> DecodedInto<T> for Decoded<T> {
    fn into(self) -> T {
        self.inner
    }
}

impl<T: DeserializeOwned> Deref for Decoded<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: DeserializeOwned> DerefMut for Decoded<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {
    use super::serde::{Deserialize, Serialize};
    use super::*;
    use crate::SimpleEncoder;

    #[derive(Serialize, Deserialize, Debug)]
    struct MyStruct {
        foo: String,
    }

    #[test]
    fn test_from_str() {
        assert_eq!(ContentType::Bson, "Bson".try_into().unwrap());
    }

    #[test]
    fn test_serialization() {
        let my_struct = MyStruct {
            foo: "bar".to_string(),
        };
        assert_eq!(
            "{\"foo\":\"bar\"}".as_bytes(),
            my_struct.encode("json").unwrap().deref()
        );
        assert_eq!(
            "{\"foo\":\"bar\"}".as_bytes(),
            my_struct.encode("application/json").unwrap().deref()
        );
    }
}
