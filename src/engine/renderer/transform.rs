use glam::{Mat4, Vec3};

pub struct Transform2D {
    pub position: [f32; 2],
    pub scale: [f32; 2]
}

impl Transform2D {
    pub fn local_to_world_model_matrix(&self) -> Mat4 {
        // from local (-0.5 - 0.5) to local (0 - 1)
        let local_to_world_local = Mat4::from_translation(Vec3::new(0.5, 0.5, 0.0));
        
        // now scale and move within world space
        let scale_transform = Mat4::from_scale(Vec3::new(self.scale[0], self.scale[1], 1.0));
        let position_transform = Mat4::from_translation(Vec3::new(self.position[0], self.position[1], 0.0));

        // executed right to left
        position_transform * scale_transform * local_to_world_local
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec4;
    use crate::engine::renderer::transform::Transform2D;

    #[test]
    fn transform2d_should_translate_local_coordinates_to_world_space() {
        // given
        let transform = Transform2D {
            position: [100.0, 100.0],
            scale: [2.0, 2.0]
        };
        let world_space_matrix = transform.local_to_world_model_matrix();
        
        // when: lower left corner in local space
        let local_space = Vec4::new(-0.5, -0.5, 0.0, 1.0);
        let world_space = world_space_matrix * local_space;
        // then
        assert_eq!(world_space, Vec4::new(100.0, 100.0, 0.0, 1.0));
        
        
        // when: center in local space
        let local_space = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let world_space = world_space_matrix * local_space;
        // then
        assert_eq!(world_space, Vec4::new(101.0, 101.0, 0.0, 1.0));
    }
}