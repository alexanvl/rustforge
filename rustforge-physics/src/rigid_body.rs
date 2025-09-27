//! Rigid body utilities

use glam::Vec3;
use rapier3d::na::{Quaternion, UnitQuaternion};
use rapier3d::prelude::*;

/// Rigid body type wrapper
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidBodyType {
    Dynamic,
    Kinematic,
    Static,
}

impl From<RigidBodyType> for rapier3d::dynamics::RigidBodyType {
    fn from(body_type: RigidBodyType) -> Self {
        match body_type {
            RigidBodyType::Dynamic => rapier3d::dynamics::RigidBodyType::Dynamic,
            RigidBodyType::Kinematic => rapier3d::dynamics::RigidBodyType::KinematicPositionBased,
            RigidBodyType::Static => rapier3d::dynamics::RigidBodyType::Fixed,
        }
    }
}

/// Create rigid body builder
pub fn create_rigid_body(
    body_type: RigidBodyType,
    position: Vec3,
    rotation: glam::Quat,
) -> RigidBodyBuilder {
    let iso = Isometry::from_parts(
        vector![position.x, position.y, position.z].into(),
        UnitQuaternion::new_normalize(Quaternion::new(
            rotation.w, rotation.x, rotation.y, rotation.z,
        )),
    );

    match body_type {
        RigidBodyType::Dynamic => RigidBodyBuilder::dynamic().position(iso).ccd_enabled(true),
        RigidBodyType::Kinematic => RigidBodyBuilder::kinematic_position_based().position(iso),
        RigidBodyType::Static => RigidBodyBuilder::fixed().position(iso),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_type_conversion() {
        assert_eq!(
            rapier3d::dynamics::RigidBodyType::from(RigidBodyType::Dynamic),
            rapier3d::dynamics::RigidBodyType::Dynamic
        );
        assert_eq!(
            rapier3d::dynamics::RigidBodyType::from(RigidBodyType::Kinematic),
            rapier3d::dynamics::RigidBodyType::KinematicPositionBased
        );
        assert_eq!(
            rapier3d::dynamics::RigidBodyType::from(RigidBodyType::Static),
            rapier3d::dynamics::RigidBodyType::Fixed
        );
    }

    #[test]
    fn test_create_dynamic_rigid_body() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let rot = glam::Quat::from_rotation_y(std::f32::consts::PI / 4.0);

        let body = create_rigid_body(RigidBodyType::Dynamic, pos, rot).build();

        // Verify body type
        assert_eq!(body.body_type(), rapier3d::dynamics::RigidBodyType::Dynamic);

        // Verify CCD is enabled for dynamic bodies
        assert!(body.is_ccd_enabled());

        // Verify position
        let translation = body.translation();
        assert_eq!(translation.x, 1.0);
        assert_eq!(translation.y, 2.0);
        assert_eq!(translation.z, 3.0);
    }

    #[test]
    fn test_create_static_rigid_body() {
        let pos = Vec3::ZERO;
        let rot = glam::Quat::IDENTITY;

        let body = create_rigid_body(RigidBodyType::Static, pos, rot).build();

        assert_eq!(body.body_type(), rapier3d::dynamics::RigidBodyType::Fixed);
        assert_eq!(body.translation(), &vector![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_create_kinematic_rigid_body() {
        let pos = Vec3::new(5.0, 10.0, -3.0);
        let rot = glam::Quat::from_rotation_x(std::f32::consts::PI / 6.0);

        let body = create_rigid_body(RigidBodyType::Kinematic, pos, rot).build();

        assert_eq!(
            body.body_type(),
            rapier3d::dynamics::RigidBodyType::KinematicPositionBased
        );

        // Verify position
        let translation = body.translation();
        assert_eq!(translation.x, 5.0);
        assert_eq!(translation.y, 10.0);
        assert_eq!(translation.z, -3.0);
    }

    #[test]
    fn test_rigid_body_rotation() {
        let pos = Vec3::ZERO;
        let rot = glam::Quat::from_rotation_y(std::f32::consts::PI / 2.0);

        let body = create_rigid_body(RigidBodyType::Dynamic, pos, rot).build();

        // Verify rotation is applied
        let rotation = body.rotation();
        assert!(rotation.w.abs() > 0.0); // Should have non-zero quaternion
    }

    #[test]
    fn test_rigid_body_ccd_enabled() {
        let pos = Vec3::ZERO;
        let rot = glam::Quat::IDENTITY;

        let dynamic_body = create_rigid_body(RigidBodyType::Dynamic, pos, rot).build();
        let static_body = create_rigid_body(RigidBodyType::Static, pos, rot).build();
        let kinematic_body = create_rigid_body(RigidBodyType::Kinematic, pos, rot).build();

        // Only dynamic bodies should have CCD enabled by default
        assert!(dynamic_body.is_ccd_enabled());
        assert!(!static_body.is_ccd_enabled());
        assert!(!kinematic_body.is_ccd_enabled());
    }

    #[test]
    fn test_rigid_body_type_enum() {
        // Test that our enum variants match expected values
        assert_eq!(RigidBodyType::Dynamic, RigidBodyType::Dynamic);
        assert_eq!(RigidBodyType::Kinematic, RigidBodyType::Kinematic);
        assert_eq!(RigidBodyType::Static, RigidBodyType::Static);

        // Test that they are different
        assert_ne!(RigidBodyType::Dynamic, RigidBodyType::Static);
        assert_ne!(RigidBodyType::Kinematic, RigidBodyType::Static);
        assert_ne!(RigidBodyType::Dynamic, RigidBodyType::Kinematic);
    }
}
