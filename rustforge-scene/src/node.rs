//! Scene graph node implementation

use glam::{Mat4, Vec3};
use rustforge_core::math::Transform;
use std::collections::HashSet;

/// Lightweight ID for scene nodes
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NodeId(pub(crate) u32);

impl NodeId {
    pub const INVALID: Self = NodeId(u32::MAX);

    pub fn is_valid(self) -> bool {
        self.0 != u32::MAX
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.0)
    }
}

/// A node in the scene graph
#[derive(Debug, Clone)]
pub struct SceneNode {
    // Core spatial data
    pub transform: Transform,
    pub(crate) world_transform: Mat4,
    pub(crate) bounds: Option<BoundingBox>,

    // Hierarchy
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: Vec<NodeId>,

    // ECS bridge - store entity ID as u32 to avoid circular dependency
    pub entity_id: Option<u32>,

    // Metadata
    pub name: String,
    pub tags: HashSet<String>,
    pub active: bool,

    // Dirty flags for optimization
    pub(crate) transform_dirty: bool,
    pub(crate) bounds_dirty: bool,
}

impl Default for SceneNode {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            world_transform: Mat4::IDENTITY,
            bounds: None,
            parent: None,
            children: Vec::new(),
            entity_id: None,
            name: String::new(),
            tags: HashSet::new(),
            active: true,
            transform_dirty: true,
            bounds_dirty: true,
        }
    }
}

impl SceneNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self.transform_dirty = true;
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    /// Get the world transform (may be outdated if dirty flag is set)
    pub fn world_transform(&self) -> &Mat4 {
        &self.world_transform
    }

    /// Get world position
    pub fn world_position(&self) -> Vec3 {
        self.world_transform.w_axis.truncate()
    }

    /// Mark transform as needing update
    pub fn mark_transform_dirty(&mut self) {
        self.transform_dirty = true;
        self.bounds_dirty = true;
    }

    /// Get children (read-only)
    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    /// Get parent
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_points(points: &[Vec3]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        let mut min = points[0];
        let mut max = points[0];

        for &point in &points[1..] {
            min = min.min(point);
            max = max.max(point);
        }

        Some(Self { min, max })
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn transform(&self, transform: &Mat4) -> Self {
        // Transform all 8 corners and find new bounds
        let corners = [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ];

        let transformed: Vec<Vec3> = corners
            .iter()
            .map(|&c| transform.transform_point3(c))
            .collect();

        Self::from_points(&transformed).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Quat;

    #[test]
    fn test_node_id() {
        let id = NodeId(42);
        assert!(id.is_valid());
        assert_eq!(format!("{}", id), "Node(42)");

        let invalid = NodeId::INVALID;
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_scene_node_creation() {
        let node = SceneNode::new("Player");
        assert_eq!(node.name, "Player");
        assert!(node.active);
        assert!(node.transform_dirty);
        assert_eq!(node.children.len(), 0);
        assert!(node.parent.is_none());
    }

    #[test]
    fn test_scene_node_builder() {
        let transform = Transform {
            position: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_rotation_y(1.57),
            scale: Vec3::splat(2.0),
        };

        let node = SceneNode::new("Enemy")
            .with_transform(transform)
            .with_tags(["enemy", "ai"]);

        assert_eq!(node.transform.position, Vec3::new(1.0, 2.0, 3.0));
        assert!(node.tags.contains("enemy"));
        assert!(node.tags.contains("ai"));
        assert_eq!(node.tags.len(), 2);
    }

    #[test]
    fn test_bounding_box() {
        let bb = BoundingBox::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(bb.center(), Vec3::ZERO);
        assert_eq!(bb.size(), Vec3::splat(2.0));
    }

    #[test]
    fn test_bounding_box_from_points() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(2.0, 1.0, 1.0),
            Vec3::new(-1.0, 2.0, 0.5),
        ];

        let bb = BoundingBox::from_points(&points).unwrap();
        assert_eq!(bb.min, Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(bb.max, Vec3::new(2.0, 2.0, 1.0));
    }

    #[test]
    fn test_bounding_box_transform() {
        let bb = BoundingBox::new(Vec3::ZERO, Vec3::ONE);
        let transform = Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0));

        let transformed = bb.transform(&transform);
        assert_eq!(transformed.min, Vec3::new(5.0, 0.0, 0.0));
        assert_eq!(transformed.max, Vec3::new(6.0, 1.0, 1.0));
    }
}
