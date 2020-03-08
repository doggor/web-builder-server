use bson;
use serde::{Deserialize, Serialize};

pub fn from_doc<'de, T>(document: bson::Document) -> Result<T, bson::DecoderError>
where T: Deserialize<'de> {
    bson::from_bson::<T>(bson::Bson::Document(document))
}

pub fn to_doc<T: Serialize>(model: &T) -> Result<bson::Document, bson::EncoderError> {
    if let bson::Bson::Document(document) = bson::to_bson(model)? {
        Ok(document)
    }
    else {
        Err(bson::EncoderError::Unknown("Cannot convert to bson::Bson::Document".to_string()))
    }
}
