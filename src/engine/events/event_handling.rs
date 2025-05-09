use crate::engine::events::Event;

trait EventHandler<T: Event> {
    fn handle(&self, event: &T);
}

struct EventDispatcher<'a, U: Event> {
    handlers: Vec<&'a dyn EventHandler<U>>
}

impl<'a, U: Event> EventDispatcher<'a, U> {

    pub fn new() -> Self {
        Self {
            handlers: Vec::new()
        }
    }

    pub fn add_event_handler(&mut self, handler: &'a dyn EventHandler<U>) {
        self.handlers.push(handler);
    }

    pub fn dispatch(&self, event: &U) {
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
    use crate::engine::events::mouse_event::MouseButtonReleasedEvent;
    use std::cell::Cell;

    #[test]
    fn event_dispatcher_should_dispatch_event_to_registered_handlers() {
        // given
        let mut event_dispatcher = EventDispatcher::<MouseButtonReleasedEvent>::new();
        let event = MouseButtonReleasedEvent{ button: MouseCode::ButtonLeft };
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

    impl<T: Event> EventHandler<T> for EventHandlerStub {
        fn handle(&self, _event: &T) {
            self.called.set(true);
        }
    }

}