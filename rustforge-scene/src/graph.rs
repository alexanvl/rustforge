//! Scene graph implementation

use crate::spatial_query::Ray;
use crate::{Frustum, NodeId, Result, SceneError, SceneNode};
use glam::{Mat4, Vec3};
use indexmap::IndexMap;
use std::collections::{HashSet, VecDeque};

/// Hierarchical scene graph
#[derive(Debug)]
pub struct SceneGraph {
    /// Node storage
    nodes: Vec<Option<SceneNode>>,

    /// Name to node mapping for fast lookup
    name_map: IndexMap<String, NodeId>,

    /// Root node
    root: NodeId,

    /// Free list for recycling node slots
    free_list: Vec<NodeId>,

    /// Next available node ID
    next_id: u32,
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneGraph {
    /// Create a new empty scene graph
    pub fn new() -> Self {
        let mut graph = Self {
            nodes: Vec::new(),
            name_map: IndexMap::new(),
            root: NodeId(0),
            free_list: Vec::new(),
            next_id: 1,
        };

        // Create root node
        let root = SceneNode::new("Root");
        graph.nodes.push(Some(root));
        graph.name_map.insert("Root".to_string(), NodeId(0));

        graph
    }

    /// Add a new node to the graph
    pub fn add_node(&mut self, mut node: SceneNode) -> NodeId {
        let id = if let Some(recycled) = self.free_list.pop() {
            recycled
        } else {
            let id = NodeId(self.next_id);
            self.next_id += 1;
            self.nodes.push(None);
            id
        };

        // Register name if not empty
        if !node.name.is_empty() {
            self.name_map.insert(node.name.clone(), id);
        }

        // Mark as needing transform update
        node.transform_dirty = true;

        self.nodes[id.0 as usize] = Some(node);
        id
    }

    /// Remove a node and all its children
    pub fn remove_node(&mut self, id: NodeId) -> Result<()> {
        if id == self.root {
            return Err(SceneError::InvalidParent(id));
        }

        // Collect all nodes to remove (node and its descendants)
        let mut to_remove = vec![id];
        let mut queue = VecDeque::new();
        queue.push_back(id);

        while let Some(current) = queue.pop_front() {
            if let Some(node) = &self.nodes[current.0 as usize] {
                for &child in &node.children {
                    to_remove.push(child);
                    queue.push_back(child);
                }
            }
        }

        // Remove from parent's children list
        if let Some(node) = &self.nodes[id.0 as usize] {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = &mut self.nodes[parent_id.0 as usize] {
                    parent.children.retain(|&child| child != id);
                }
            }
        }

        // Remove all nodes
        for remove_id in to_remove {
            if let Some(node) = self.nodes[remove_id.0 as usize].take() {
                if !node.name.is_empty() {
                    self.name_map.shift_remove(&node.name);
                }
                self.free_list.push(remove_id);
            }
        }

        Ok(())
    }

    /// Get a node by ID
    pub fn get(&self, id: NodeId) -> Result<&SceneNode> {
        self.nodes
            .get(id.0 as usize)
            .and_then(|n| n.as_ref())
            .ok_or(SceneError::NodeNotFound(id))
    }

    /// Get a mutable node by ID
    pub fn get_mut(&mut self, id: NodeId) -> Result<&mut SceneNode> {
        self.nodes
            .get_mut(id.0 as usize)
            .and_then(|n| n.as_mut())
            .ok_or(SceneError::NodeNotFound(id))
    }

    /// Find a node by name
    pub fn find_by_name(&self, name: &str) -> Option<NodeId> {
        self.name_map.get(name).copied()
    }

    /// Add a child to a parent node
    pub fn add_child(&mut self, parent: NodeId, child: NodeId) -> Result<()> {
        if parent == child {
            return Err(SceneError::CircularDependency);
        }

        // Check for circular dependency
        if self.is_ancestor(child, parent)? {
            return Err(SceneError::CircularDependency);
        }

        // Remove from previous parent
        if let Some(node) = &self.nodes[child.0 as usize] {
            if let Some(old_parent) = node.parent {
                if let Some(old_parent_node) = &mut self.nodes[old_parent.0 as usize] {
                    old_parent_node.children.retain(|&c| c != child);
                }
            }
        }

        // Add to new parent
        let parent_node = self.get_mut(parent)?;
        parent_node.children.push(child);

        // Update child's parent
        let child_node = self.get_mut(child)?;
        child_node.parent = Some(parent);
        child_node.transform_dirty = true;

        Ok(())
    }

    /// Remove a child from its parent
    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) -> Result<()> {
        let parent_node = self.get_mut(parent)?;
        parent_node.children.retain(|&c| c != child);

        let child_node = self.get_mut(child)?;
        child_node.parent = None;
        child_node.transform_dirty = true;

        Ok(())
    }

    /// Set the local position of a node
    pub fn set_position(&mut self, id: NodeId, position: Vec3) -> Result<()> {
        let node = self.get_mut(id)?;
        node.transform.position = position;
        node.mark_transform_dirty();
        Ok(())
    }

    /// Set the local rotation of a node
    pub fn set_rotation(&mut self, id: NodeId, rotation: glam::Quat) -> Result<()> {
        let node = self.get_mut(id)?;
        node.transform.rotation = rotation;
        node.mark_transform_dirty();
        Ok(())
    }

    /// Set the local scale of a node
    pub fn set_scale(&mut self, id: NodeId, scale: Vec3) -> Result<()> {
        let node = self.get_mut(id)?;
        node.transform.scale = scale;
        node.mark_transform_dirty();
        Ok(())
    }

    /// Update all transforms in the graph
    pub fn update_transforms(&mut self) {
        self.update_node_transforms(self.root, None);
    }

    /// Update transforms for a specific node and its children
    fn update_node_transforms(&mut self, id: NodeId, parent_world: Option<&Mat4>) {
        let node = match self.nodes.get_mut(id.0 as usize).and_then(|n| n.as_mut()) {
            Some(n) => n,
            None => return,
        };

        // Update world transform if needed
        if node.transform_dirty || parent_world.is_some() {
            let local = node.transform.matrix();
            node.world_transform = match parent_world {
                Some(parent) => *parent * local,
                None => local,
            };
            node.transform_dirty = false;
        }

        // Get children and world transform for recursion
        let children = node.children.clone();
        let world = node.world_transform;

        // Update children
        for child_id in children {
            self.update_node_transforms(child_id, Some(&world));
        }
    }

    /// Check if a node is an ancestor of another
    fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> Result<bool> {
        let mut current = descendant;
        let mut visited = HashSet::new();

        while let Some(node) = self.nodes.get(current.0 as usize).and_then(|n| n.as_ref()) {
            if !visited.insert(current) {
                return Err(SceneError::CircularDependency);
            }

            if let Some(parent) = node.parent {
                if parent == ancestor {
                    return Ok(true);
                }
                current = parent;
            } else {
                break;
            }
        }

        Ok(false)
    }

    /// Query nodes within a frustum
    pub fn query_frustum(&self, frustum: &Frustum) -> Vec<NodeId> {
        let mut results = Vec::new();
        self.query_frustum_recursive(self.root, frustum, &mut results);
        results
    }

    fn query_frustum_recursive(&self, id: NodeId, frustum: &Frustum, results: &mut Vec<NodeId>) {
        let node = match self.nodes.get(id.0 as usize).and_then(|n| n.as_ref()) {
            Some(n) if n.active => n,
            _ => return,
        };

        // Test node bounds if available
        let mut in_frustum = true;
        if let Some(bounds) = &node.bounds {
            let world_bounds = bounds.transform(&node.world_transform);
            in_frustum = frustum.intersects_box(&world_bounds);
        }

        if in_frustum {
            results.push(id);

            // Test children
            for &child in &node.children {
                self.query_frustum_recursive(child, frustum, results);
            }
        }
    }

    /// Find the closest node hit by a ray
    pub fn raycast(&self, ray: &Ray) -> Option<(NodeId, f32)> {
        let mut closest: Option<(NodeId, f32)> = None;
        self.raycast_recursive(self.root, ray, &mut closest);
        closest
    }

    fn raycast_recursive(&self, id: NodeId, ray: &Ray, closest: &mut Option<(NodeId, f32)>) {
        let node = match self.nodes.get(id.0 as usize).and_then(|n| n.as_ref()) {
            Some(n) if n.active => n,
            _ => return,
        };

        // Test node bounds
        if let Some(bounds) = &node.bounds {
            let world_bounds = bounds.transform(&node.world_transform);
            if let Some(t) = ray.intersects_box(&world_bounds) {
                match closest {
                    Some((_, closest_t)) if t < *closest_t => {
                        *closest = Some((id, t));
                    }
                    None => {
                        *closest = Some((id, t));
                    }
                    _ => {}
                }
            }
        }

        // Test children
        for &child in &node.children {
            self.raycast_recursive(child, ray, closest);
        }
    }

    /// Get root node ID
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Iterate over all nodes
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &SceneNode)> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| node.as_ref().map(|n| (NodeId(i as u32), n)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_graph_creation() {
        let graph = SceneGraph::new();
        assert_eq!(graph.root(), NodeId(0));

        let root = graph.get(graph.root()).unwrap();
        assert_eq!(root.name, "Root");
    }

    #[test]
    fn test_add_remove_nodes() {
        let mut graph = SceneGraph::new();

        let _player_id = graph.add_node(SceneNode::new("Player"));
        let weapon_id = graph.add_node(SceneNode::new("Weapon"));

        assert!(graph.find_by_name("Player").is_some());
        assert!(graph.find_by_name("Weapon").is_some());

        graph.remove_node(weapon_id).unwrap();
        assert!(graph.find_by_name("Weapon").is_none());
        assert!(graph.get(weapon_id).is_err());
    }

    #[test]
    fn test_hierarchy() {
        let mut graph = SceneGraph::new();

        let parent_id = graph.add_node(SceneNode::new("Parent"));
        let child1_id = graph.add_node(SceneNode::new("Child1"));
        let child2_id = graph.add_node(SceneNode::new("Child2"));

        graph.add_child(parent_id, child1_id).unwrap();
        graph.add_child(parent_id, child2_id).unwrap();

        let parent = graph.get(parent_id).unwrap();
        assert_eq!(parent.children.len(), 2);

        let child1 = graph.get(child1_id).unwrap();
        assert_eq!(child1.parent, Some(parent_id));
    }

    #[test]
    fn test_circular_dependency_prevention() {
        let mut graph = SceneGraph::new();

        let a = graph.add_node(SceneNode::new("A"));
        let b = graph.add_node(SceneNode::new("B"));
        let c = graph.add_node(SceneNode::new("C"));

        graph.add_child(a, b).unwrap();
        graph.add_child(b, c).unwrap();

        // Try to create circular dependency
        let result = graph.add_child(c, a);
        assert!(matches!(result, Err(SceneError::CircularDependency)));
    }

    #[test]
    fn test_transform_propagation() {
        let mut graph = SceneGraph::new();

        let parent_id = graph.add_node(SceneNode::new("Parent"));
        let child_id = graph.add_node(SceneNode::new("Child"));

        graph.add_child(graph.root(), parent_id).unwrap();
        graph.add_child(parent_id, child_id).unwrap();
        graph
            .set_position(parent_id, Vec3::new(10.0, 0.0, 0.0))
            .unwrap();
        graph
            .set_position(child_id, Vec3::new(5.0, 0.0, 0.0))
            .unwrap();

        graph.update_transforms();

        let child = graph.get(child_id).unwrap();
        let world_pos = child.world_position();
        assert_eq!(world_pos, Vec3::new(15.0, 0.0, 0.0));
    }

    #[test]
    fn test_node_recycling() {
        let mut graph = SceneGraph::new();

        let id1 = graph.add_node(SceneNode::new("Node1"));
        graph.remove_node(id1).unwrap();

        let id2 = graph.add_node(SceneNode::new("Node2"));
        // Should reuse the same slot
        assert_eq!(id1, id2);
    }
}
