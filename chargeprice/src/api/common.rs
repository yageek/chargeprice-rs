use std::ops::Deref;

use serde::Deserialize;

/// The envelope for the response
#[derive(Debug, Deserialize)]
pub struct Response<T, R> {
    data: Vec<Entity<T, R>>,
}

impl<T, R> Response<T, R> {
    pub fn data(&self) -> &[Entity<T, R>] {
        &self.data
    }
}

/// Some internal inner wrapper
#[derive(Debug, Deserialize)]
pub(super) struct InnerData<T> {
    pub data: T,
}

impl<T> Deref for InnerData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// A entity referernce generally used within relat
#[derive(Debug, Deserialize)]
pub(super) struct EntityRef {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub kind: String,
}

/// The common structure of every API object
#[derive(Debug, Deserialize)]
pub struct Entity<T, R> {
    #[serde(flatten)]
    reference: EntityRef,
    attributes: T,
    relationships: Option<R>,
}

impl<T, R> Entity<T, R> {
    /// The id of the entity
    pub fn id(&self) -> &str {
        &self.reference.id
    }

    pub fn relationships(&self) -> Option<&R> {
        self.relationships.as_ref()
    }

    pub fn attributes(&self) -> &T {
        &self.attributes
    }
}

#[derive(Debug, Deserialize)]
pub struct Error {
    title: String,
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    errors: Vec<Error>,
}
