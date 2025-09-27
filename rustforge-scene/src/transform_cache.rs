//! Transform caching and optimization

use crate::NodeId;
use glam::Mat4;
use std::collections::HashMap;

/// Cache for world transforms to avoid recalculation
#[derive(Debug, Default)]
pub struct TransformCache {
    /// Cached world transforms
    transforms: HashMap<NodeId, Mat4>,

    /// Generation counter for cache invalidation
    generation: u64,

    /// Node generation stamps
    node_generations: HashMap<NodeId, u64>,
}

impl TransformCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a cached transform
    pub fn get(&self, id: NodeId) -> Option<&Mat4> {
        self.transforms.get(&id)
    }

    /// Update a cached transform
    pub fn update(&mut self, id: NodeId, transform: Mat4) {
        self.transforms.insert(id, transform);
        self.node_generations.insert(id, self.generation);
    }

    /// Invalidate a node's cache and its descendants
    pub fn invalidate(&mut self, id: NodeId) {
        self.generation += 1;
        self.node_generations.remove(&id);
        self.transforms.remove(&id);
    }

    /// Check if a node's cache is valid
    pub fn is_valid(&self, id: NodeId) -> bool {
        self.node_generations
            .get(&id)
            .map(|&gen| gen == self.generation)
            .unwrap_or(false)
    }

    /// Clear all cached data
    pub fn clear(&mut self) {
        self.transforms.clear();
        self.node_generations.clear();
        self.generation += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_cache() {
        let mut cache = TransformCache::new();
        let node_id = NodeId(1);
        let transform = Mat4::from_translation(glam::Vec3::new(1.0, 2.0, 3.0));

        cache.update(node_id, transform);
        assert!(cache.is_valid(node_id));
        assert_eq!(cache.get(node_id), Some(&transform));

        cache.invalidate(node_id);
        assert!(!cache.is_valid(node_id));
        assert!(cache.get(node_id).is_none());
    }
}
