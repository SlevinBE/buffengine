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