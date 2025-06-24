use log::debug;
use crate::engine::core::layer::Layer;
use crate::engine::core::scene::Scene;
use crate::engine::events::Event;
use crate::engine::gameobjects::GameObject;
use crate::sample_game::sample_game_object::SampleGameObject;

pub struct SceneLayer {
    pub name: String,
    game_objects: Vec<Box<dyn GameObject>>
}

impl SceneLayer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            game_objects: vec![
                Box::new(SampleGameObject::new())
            ]
        }
    }
}

impl Layer for SceneLayer {

    fn update(&self) {
        debug!("SampleLayer update");
    }

    fn handle_event(&self, event: &Box<dyn Event>) -> bool {
        debug!("SampleLayer event: {:?}", event.get_event_type());
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
    fn get_game_objects(&self) -> &[Box<dyn GameObject>] {
        self.game_objects.as_slice()
    }
}