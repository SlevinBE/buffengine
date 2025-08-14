use std::any::Any;
use std::cell::{Cell, Ref, RefCell};
use std::ops::{Deref, DerefMut};
use log::debug;
use crate::engine::core::key_codes::KeyCode;
use crate::engine::core::layer::Layer;
use crate::engine::events::{ApplicationEvent, Event, KeyboardEvent};
use crate::engine::events::ApplicationEvent::WindowResized;
use crate::engine::renderer::{Renderable, Scene};
use crate::engine::renderer::camera::Camera2D;
use crate::sample_game::sprite::Sprite;
use crate::sample_game::sprite::Movable;

pub struct SceneLayer {
    pub name: String,
    player: RefCell<Sprite>,
    npcs: Vec<Sprite>,
    camera: RefCell<Camera2D>
}

impl SceneLayer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            player: RefCell::new(Sprite::new(0.0, 1.0, 5.0)),
            npcs: vec![
                Sprite::new(2.0, 7.0, 2.0),
                Sprite::new(7.0, 3.0, 1.0)
            ],
            camera: RefCell::new(Camera2D {
                position: [0.0, 0.0],
                size: [25.0, 25.0],
                viewport_size: [1024, 768]
            })
        }
    }
}

impl Layer for SceneLayer {

    fn update(&self) {
        debug!("SampleLayer update");
    }

    fn handle_event(&self, event: &Event) -> bool {
        debug!("SampleLayer event: {:?}", event);
        match event {
            Event::ApplicationEvent(WindowResized { width, height}) => {
                self.camera.borrow_mut().update_viewport_size([*width, *height]);
                true
            },
            Event::KeyboardEvent(KeyboardEvent::KeyPressed { key_code, is_repeat }) => {
                match *key_code {
                    KeyCode::Up => {
                        self.player.borrow_mut().move_up(1.0);
                        true
                    },
                    KeyCode::Down => {
                        self.player.borrow_mut().move_down(1.0);
                        true
                    },
                    KeyCode::Left => {
                        self.player.borrow_mut().move_left(1.0);
                        true
                    },
                    KeyCode::Right => {
                        self.player.borrow_mut().move_right(1.0);
                        true
                    },
                    _ => false
                }
            },
            _ => false
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn as_scene(&self) -> Option<&dyn Scene> {
        Some(self)
    }
}

impl Scene for SceneLayer {
    fn get_renderables(&self) -> Vec<Renderable> {
        let mut renderables = self.npcs.iter().map(|go| {
            go.get_renderable().clone()
        }).collect::<Vec<_>>();
        renderables.push(self.player.borrow().deref().get_renderable().clone());
        renderables
    }

    fn get_camera(&self) -> Ref<Camera2D> {
        self.camera.borrow()
    }
}