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
extern crate serde_derive;
extern crate serde_json;
extern crate serde_lexpr;
extern crate serde_pickle;
extern crate serde_qs;
extern crate serde_xml_rs;
extern crate serde_yaml;

pub mod prelude {
    pub extern crate bson;
    pub extern crate flexbuffers;
    pub extern crate json5;
    pub extern crate postcard;
    pub extern crate rmp_serde as messagepack;
    pub extern crate ron;
    pub extern crate serde_cbor as cbor;
    pub extern crate serde_json as json;
    pub extern crate serde_lexpr as lexpr;
    pub extern crate serde_pickle as pickle;
    pub extern crate serde_qs as url;
    pub extern crate serde_xml_rs as xml;
    pub extern crate serde_yaml as yaml;
}
#[cfg(feature = "http")]
use http::{header::ToStrError, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::{Infallible, TryFrom, TryInto};
use std::str::Utf8Error;

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
            "pickle" => Ok(ContentType::Pickle),
            "postcard" => Ok(ContentType::Postcard),
            "ron" => Ok(ContentType::Ron),
            "application/ron" => Ok(ContentType::Ron),
            "application/x-ron" => Ok(ContentType::Ron),
            "toml" => Ok(ContentType::Toml),
            "application/toml" => Ok(ContentType::Toml),
            "application/x-toml" => Ok(ContentType::Toml),
            "url" => Ok(ContentType::Url),
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

#[cfg(feature = "http")]
impl TryFrom<HeaderValue> for ContentType {
    type Error = crate::Error;

    fn try_from(h: HeaderValue) -> std::result::Result<ContentType, Self::Error> {
        h.to_str()
            .map_err(Error::from)
            .and_then(ContentType::try_from)
    }
}

#[derive(Debug)]
pub enum Error {
    Infallible,
    ByteToUTF8ConversionFailure(Utf8Error),
    UnknownContentTypeMatchFromStr(String),
    BsonSerializationFailure(bson::ser::Error),
    BsonDeserializationFailure(bson::de::Error),
    CborFailure(serde_cbor::Error),
    FlexBuffersSerializationFailure(flexbuffers::SerializationError),
    FlexBuffersDeserializationFailure(flexbuffers::DeserializationError),
    JsonError(serde_json::Error),
    Json5Error(json5::Error),
    LexprError(serde_lexpr::Error),
    MessagePackEncodeError(rmp_serde::encode::Error),
    MessagePackDecodeError(rmp_serde::decode::Error),
    PickleError(serde_pickle::Error),
    PostcardError(postcard::Error),
    RonError(ron::Error),
    TomlSerializationFailure(toml::ser::Error),
    TomlDeserializationFailure(toml::de::Error),
    UrlEncodingFailure(serde_qs::Error),
    YamlError(serde_yaml::Error),
    XmlError(serde_xml_rs::Error),
    TypeDoesNotSupportSerialization(ContentType),
    #[cfg(feature = "http")]
    FailedConvertingHeaderValueToContentType(http::header::ToStrError),
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

pub trait SimpleEncoder
where
    Self: serde::Serialize,
{
    fn encode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<Vec<u8>>;
}

impl<T> SimpleEncoder for T
where
    T: Serialize,
{
    fn encode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<Vec<u8>> {
        use std::str::from_utf8;
        let bson = |o: &T| -> Result<Vec<u8>> { bson::to_vec(o).map_err(Error::from) };
        let cbor = |o: &T| -> Result<Vec<u8>> { serde_cbor::to_vec(o).map_err(Error::from) };
        let flexbuffers =
            |o: &T| -> Result<Vec<u8>> { flexbuffers::to_vec(o).map_err(Error::from) };
        let json = |o: &T| -> Result<Vec<u8>> { serde_json::to_vec(o).map_err(Error::from) };
        let json5 = |o: &T| -> Result<Vec<u8>> {
            json5::to_string(o)
                .map(|s| s.as_bytes().to_vec())
                .map_err(Error::from)
        };
        let lexpr = |o: &T| -> Result<Vec<u8>> { serde_lexpr::to_vec(o).map_err(Error::from) };
        let message_pack = |o: &T| -> Result<Vec<u8>> { rmp_serde::to_vec(o).map_err(Error::from) };
        let pickle = |o: &T| -> Result<Vec<u8>> {
            serde_pickle::to_vec(o, Default::default()).map_err(Error::from)
        };
        let postcard = |o: &T| -> Result<Vec<u8>> { postcard::to_allocvec(o).map_err(Error::from) };
        let ron = |o: &T| -> Result<Vec<u8>> {
            ron::to_string(o)
                .map(|s| s.as_bytes().to_vec())
                .map_err(Error::from)
        };
        let toml = |o: &T| -> Result<Vec<u8>> { toml::to_vec(o).map_err(Error::from) };
        let url = |o: &T| -> Result<Vec<u8>> {
            serde_qs::to_string(o)
                .map(|s| s.as_bytes().to_vec())
                .map_err(Error::from)
        };
        let yaml = |o: &T| -> Result<Vec<u8>> { serde_yaml::to_vec(o).map_err(Error::from) };
        let xml = |o: &T| -> Result<Vec<u8>> {
            serde_xml_rs::to_string(o)
                .map(|s| s.as_bytes().to_vec())
                .map_err(Error::from)
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

pub trait SimpleDecoder<T> {
    fn decode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<T>;
}

impl<T> SimpleDecoder<T> for &[u8]
where
    T: DeserializeOwned,
{
    fn decode<F: TryInto<ContentType, Error = impl Into<crate::Error>>>(
        &self,
        content_type: F,
    ) -> Result<T> {
        let bson = |o: &[u8]| -> Result<T> { bson::from_slice(o).map_err(Error::from) };
        let cbor = |o: &[u8]| -> Result<T> { serde_cbor::from_slice(o).map_err(Error::from) };
        let flexbuffers =
            |o: &[u8]| -> Result<T> { flexbuffers::from_slice(o).map_err(Error::from) };
        let json = |o: &[u8]| -> Result<T> { serde_json::from_slice(o).map_err(Error::from) };
        let json5 = |o: &[u8]| -> Result<T> {
            std::str::from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| json5::from_str(str).map_err(Error::from))
        };
        let lexpr = |o: &[u8]| -> Result<T> { serde_lexpr::from_slice(o).map_err(Error::from) };
        let message_pack =
            |o: &[u8]| -> Result<T> { rmp_serde::from_slice(o).map_err(Error::from) };
        let pickle = |o: &[u8]| -> Result<T> {
            serde_pickle::from_slice(o, Default::default()).map_err(Error::from)
        };
        let postcard = |o: &[u8]| -> Result<T> { postcard::from_bytes(o).map_err(Error::from) };
        let ron = |o: &[u8]| -> Result<T> {
            std::str::from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| ron::from_str(str).map_err(Error::from))
        };
        let toml = |o: &[u8]| -> Result<T> { toml::from_slice(o).map_err(Error::from) };
        let url = |o: &[u8]| -> Result<T> { serde_qs::from_bytes(o).map_err(Error::from) };
        let yaml = |o: &[u8]| -> Result<T> { serde_yaml::from_slice(o).map_err(Error::from) };
        let xml = |o: &[u8]| -> Result<T> {
            std::str::from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| serde_xml_rs::from_str(str).map_err(Error::from))
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
            my_struct.encode("json").unwrap()
        );
        assert_eq!(
            "{\"foo\":\"bar\"}".as_bytes(),
            my_struct.encode("application/json").unwrap()
        );
    }
}
