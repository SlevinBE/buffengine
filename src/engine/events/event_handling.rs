use crate::engine::events::Event;

trait EventHandler {
    fn handle(&self, event: &Event);
}

struct EventDispatcher<'a> {
    handlers: Vec<&'a dyn EventHandler>
}

impl<'a> EventDispatcher<'a> {

    pub fn new() -> Self {
        Self {
            handlers: Vec::new()
        }
    }

    pub fn add_event_handler(&mut self, handler: &'a dyn EventHandler) {
        self.handlers.push(handler);
    }

    pub fn dispatch(&self, event: &Event) {
        for handler in &self.handlers {
            handler.handle(event)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::core::mouse_codes::MouseCode;
    use crate::engine::events::event_handling::EventDispatcher;
    use std::cell::Cell;
    use crate::engine::events::MouseEvent::MouseButtonPressed;

    #[test]
    fn event_dispatcher_should_dispatch_event_to_registered_handlers() {
        // given
        let mut event_dispatcher = EventDispatcher::new();
        let event = Event::MouseEvent(MouseButtonPressed { button: MouseCode::ButtonLeft });
        let event_handler = EventHandlerStub::new();
        event_dispatcher.add_event_handler(&event_handler);

        // when
        event_dispatcher.dispatch(&event);

        // then
        assert!(event_handler.was_called());
    }

    struct EventHandlerStub {
        called: Cell<bool>
    }

    impl EventHandlerStub {
        fn new() -> Self {
            Self {
                called: Cell::new(false)
            }
        }

        pub fn was_called(&self) -> bool {
            self.called.get()
        }
    }

    impl EventHandler for EventHandlerStub {
        fn handle(&self, _event: &Event) {
            self.called.set(true);
        }
    }

}