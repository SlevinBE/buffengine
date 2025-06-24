use crate::engine::gameobjects::GameObject;

pub trait Scene {
    fn get_game_objects(&self) -> &[Box<dyn GameObject>];
}