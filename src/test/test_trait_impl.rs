use super::super::Error;
#[cfg(not(tarpaulin_include))]
impl Eq for Error {}
#[cfg(not(tarpaulin_include))]
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::Infallible => match other {
                Error::Infallible => true,
                _ => false,
            },
            Error::ByteToUTF8ConversionFailure(e) => match other {
                Error::ByteToUTF8ConversionFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::UnknownContentTypeMatchFromStr(e) => match other {
                Error::UnknownContentTypeMatchFromStr(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::BsonSerializationFailure(e) => match other {
                Error::BsonSerializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::BsonDeserializationFailure(e) => match other {
                Error::BsonDeserializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::CborFailure(e) => match other {
                Error::CborFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::FlexBuffersSerializationFailure(e) => match other {
                Error::FlexBuffersSerializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::FlexBuffersDeserializationFailure(e) => match other {
                Error::FlexBuffersDeserializationFailure(ee) => {
                    format!("{}", e) == format!("{}", ee)
                }
                _ => false,
            },
            Error::JsonError(e) => match other {
                Error::JsonError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::Json5Error(e) => match other {
                Error::Json5Error(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::LexprError(e) => match other {
                Error::LexprError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::MessagePackEncodeError(e) => match other {
                Error::MessagePackEncodeError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::MessagePackDecodeError(e) => match other {
                Error::MessagePackDecodeError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::PickleError(e) => match other {
                Error::PickleError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::PostcardError(e) => match other {
                Error::PostcardError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::RonError(e) => match other {
                Error::RonError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::TomlSerializationFailure(e) => match other {
                Error::TomlSerializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::TomlDeserializationFailure(e) => match other {
                Error::TomlDeserializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::UrlEncodingFailure(e) => match other {
                Error::UrlEncodingFailure(ee) => format!("{:?}", e) == format!("{:?}", ee),
                _ => false,
            },
            Error::YamlError(e) => match other {
                Error::YamlError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "accept-limited-xml-serialize")]
            Error::XmlError(e) => match other {
                Error::XmlError(ee) => format!("{:?}", e) == format!("{:?}", ee),
                _ => false,
            },
            Error::TypeDoesNotSupportSerialization(e) => match other {
                Error::TypeDoesNotSupportSerialization(ee) => {
                    format!("{:?}", e) == format!("{:?}", ee)
                }
                _ => false,
            },
            #[cfg(feature = "http")]
            Error::FailedConvertingHeaderValueToContentType(e) => match other {
                Error::FailedConvertingHeaderValueToContentType(ee) => {
                    format!("{}", e) == format!("{}", ee)
                }
                _ => false,
            },
            #[cfg(feature = "http")]
            Error::InvalidHeaderValue(e) => match other {
                Error::InvalidHeaderValue(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::RonDecodeError(e) => match other {
                Error::RonDecodeError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
        }
    }
}
