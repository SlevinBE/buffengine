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

#[cfg(test)]
mod tests {
    use glam::{Mat4, Vec4};
    use crate::engine::renderer::camera::Camera2D;

    #[test]
    fn camera2d_should_translate_world_coordinates_to_view_space() {
        // given
        let camera = Camera2D {
            position: [8.0, 8.0],
            size: [10.0, 10.0],
            viewport_size: [10, 10]
        };
        let world_position = Vec4::new(10.0, 10.0, 0.0, 1.0);
        
        // when
        let world_to_view_matrix: Mat4 = camera.world_to_view_matrix();
        let view_position: Vec4 = world_to_view_matrix * world_position;
        
        // then: position has shifted 8 positions across the x/y-axis to the lower/bottom left of the camera
        assert_eq!(view_position, Vec4::new(2.0, 2.0, 0.0, 1.0));
    }
    
    #[test]
    fn camera2d_should_translate_view_coordinates_to_clip_space_when_position_inside_camera_bounds() {
        // given
        let camera = Camera2D {
            position: [0.0, 0.0],
            size: [10.0, 10.0],
            viewport_size: [10, 10] // the same as size to keep a 1:1 aspect ratio
        };
        let view_position = Vec4::new(5.0, 5.0, 0.0, 1.0);
        
        // when
        let view_to_clip_matrix = camera.view_to_clip_matrix();
        let clip_position = view_to_clip_matrix * view_position;
        
        // then
        assert_eq!(clip_position, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn camera2d_should_translate_view_coordinates_to_clip_space_when_position_outside_camera_bounds() {
        // given
        let camera = Camera2D {
            position: [0.0, 0.0],
            size: [10.0, 10.0],
            viewport_size: [10, 10] // the same as size to keep a 1:1 aspect ratio
        };
        let view_position = Vec4::new(15.0, 15.0, 0.0, 1.0);

        // when
        let view_to_clip_matrix = camera.view_to_clip_matrix();
        let clip_position = view_to_clip_matrix * view_position;

        // then
        assert_eq!(clip_position, Vec4::new(2.0, 2.0, 0.0, 1.0));
    }
}