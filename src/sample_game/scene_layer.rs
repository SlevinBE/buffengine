use std::any::Any;
use std::cell::{Ref, RefCell};
use log::debug;
use crate::engine::core::layer::Layer;
use crate::engine::events::{Event, EventType};
use crate::engine::events::application_event::WindowResizeEvent;
use crate::engine::events::EventType::WindowResize;
use crate::engine::gameobjects::GameObject;
use crate::engine::renderer::{Renderable, Scene};
use crate::engine::renderer::camera::Camera2D;
use crate::sample_game::sample_game_object::SampleGameObject;

pub struct SceneLayer {
    pub name: String,
    game_objects: Vec<Box<dyn GameObject>>,
    camera: RefCell<Camera2D>
}

impl SceneLayer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            game_objects: vec![
                Box::new(SampleGameObject::new(0.0, 1.0, 5.0)),
                Box::new(SampleGameObject::new(2.0, 7.0, 2.0)),
                Box::new(SampleGameObject::new(7.0, 3.0, 1.0))
            ],
            camera: RefCell::new(Camera2D {
                position: [0.0, 0.0],
                size: [10.0, 10.0],
                viewport_size: [1024, 768]
            })
        }
    }
}

impl Layer for SceneLayer {

    fn update(&self) {
        debug!("SampleLayer update");
    }

    fn handle_event(&self, event: &Box<dyn Event>) -> bool {
        debug!("SampleLayer event: {:?}", event.get_event_type());
        match event.get_event_type() {
            WindowResize => {
                let event = event.as_any().downcast_ref::<WindowResizeEvent>().unwrap();
                self.camera.borrow_mut().update_viewport_size([event.width, event.height])
            },
            _ => { // ignore 
            }
        }
        true
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn as_scene(&self) -> Option<&dyn Scene> {
        Some(self)
    }
}

impl Scene for SceneLayer {
    fn get_renderables(&self) -> Vec<&Renderable> {
        self.game_objects.iter().map(|go| {
            go.get_renderable()
        }).collect::<Vec<_>>()
    }

    fn get_camera(&self) -> Ref<Camera2D> {
        self.camera.borrow()
    }
}