//! Scene graph implementation for RustForge
//!
//! This crate provides a hierarchical scene graph for organizing game objects
//! with spatial relationships and efficient transform propagation.

mod graph;
mod node;
mod serialization;
mod spatial_query;
mod transform_cache;

pub use graph::SceneGraph;
pub use node::{NodeId, SceneNode};
pub use serialization::{ComponentData, NodeData, SceneFile};
pub use spatial_query::{BoundingBox, Frustum};
pub use transform_cache::TransformCache;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("Node not found: {0:?}")]
    NodeNotFound(NodeId),

    #[error("Invalid parent: {0:?}")]
    InvalidParent(NodeId),

    #[error("Circular dependency detected")]
    CircularDependency,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("RON error: {0}")]
    RonError(#[from] ron::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SceneError>;

pub mod prelude {
    pub use crate::{BoundingBox, Frustum};
    pub use crate::{ComponentData, NodeData, SceneFile};
    pub use crate::{NodeId, Result, SceneError, SceneGraph, SceneNode};
}
