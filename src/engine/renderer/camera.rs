use glam::{Mat4, Vec3};

pub struct Camera2D {
    pub position: [f32; 2], // center position in world units
    pub size: [f32; 2], // world units
    pub viewport_size: [u32; 2] // screen pixels
}

impl Camera2D {
    
    pub fn update_viewport_size(&mut self, new_size: [u32; 2]) {
        self.viewport_size = new_size;
    }
    
    pub fn world_to_view_matrix(&self) -> Mat4 {
        // from world space to screen space
        // translate world coordinates so that the camera position becomes the origin (0, 0) in view space
        Mat4::from_translation(Vec3::new(-self.position[0], -self.position[1], 0.0))
    }
    
    pub fn view_to_clip_matrix(&self) -> Mat4 {
        let aspect_ratio = self.viewport_size[0] / self.viewport_size[1];
        
        let height = self.size[1];
        let width = height * (aspect_ratio as f32);
        
        let left = 0.0;
        let right = width;
        let bottom = 0.0;
        let top = height;
        let near = 0.0;
        let far = 100.0;
        
        Mat4::orthographic_lh(left, right, bottom, top, near, far)
    }
}