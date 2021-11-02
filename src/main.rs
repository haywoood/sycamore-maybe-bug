use log::debug;
use log::Level;
use std::cell::*;
use std::collections::HashMap;
use std::rc::*;
use sycamore::generic_node::*;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type PostEventCallback = Box<dyn FnOnce()>;

#[derive(Debug)]
enum Event {
    InitializeDB,
}

#[derive(Clone, Copy, Debug)]
enum Trigger {
    AddEvent,
    RunQueue,
}

#[derive(Clone, Copy, Debug)]
enum FsmState {
    Idle,
    Scheduled,
    Running,
    Paused,
}

#[derive(Clone)]
struct EventQueue {
    fsm_state: Rc<RefCell<FsmState>>,
    queue: Rc<RefCell<Vec<Option<Event>>>>,
    post_event_callback_fns: Rc<RefCell<HashMap<String, PostEventCallback>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            fsm_state: Rc::new(RefCell::new(FsmState::Idle)),
            queue: Rc::new(RefCell::new(vec![])),
            post_event_callback_fns: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    fn fsm_trigger(&self, trigger: Trigger, event: Option<Event>) {
        debug!("Trigger: {:?}", trigger);
        debug!("Event: {:?}", event);
        let (new_fsm_state, action_fn): (FsmState, Option<Box<dyn FnOnce()>>) = {
            let current_fsm_state = *self.fsm_state.borrow();
            match (current_fsm_state, trigger) {
                (FsmState::Idle, Trigger::AddEvent) => (
                    FsmState::Scheduled,
                    Some(Box::new(move || {
                        self.add_event(event);
                        self.run_next_tick();
                    })),
                ),
                _ => (FsmState::Scheduled, Some(Box::new(|| {}))),
            }
        };

        self.fsm_state.replace(new_fsm_state);

        match action_fn {
            Some(action_fn) => action_fn(),
            None => (),
        }
    }

    fn push(&self, event: Event) {
        self.fsm_trigger(Trigger::AddEvent, Some(event))
    }

    fn add_event(&self, event: Option<Event>) {
        self.queue.borrow_mut().push(event)
    }

    fn next_tick(&self, f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .expect("no global `window` exists")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("wtf");
    }

    fn run_next_tick(&self) {
        let self_cloned = self.clone();
        let next_tick = Closure::wrap(Box::new(move || {
            debug!("I'm in the closure");
            self_cloned.fsm_trigger(Trigger::RunQueue, None)
        }) as Box<dyn FnMut()>);
        self.next_tick(&next_tick);
        // we need to forget lest we want our closure to get dropped.
        // This can be a source of memory leaks CARE
        next_tick.forget()
    }
}

thread_local! {
    static EVENT_QUEUE: RefCell<EventQueue> = RefCell::new(EventQueue::new());
}

fn dispatch(event: Event) {
    EVENT_QUEUE.with(|event_queue| {
        event_queue.borrow_mut().push(event);
    })
}

fn main() {
    console_log::init_with_level(Level::Debug);

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = &document.body().unwrap();

    dispatch(Event::InitializeDB);

    let x = EventQueue::new();
    x.fsm_state.replace(FsmState::Scheduled);
    debug!("x: {:?}", x.fsm_state.borrow());
    let y = x.clone();
    y.fsm_state.replace(FsmState::Running);
    debug!("y: {:?}", y.fsm_state.borrow());
    debug!("x: {:?}", x.fsm_state.borrow());

    render_plain(
        || {
            template! {
                p { "coca~colas" }
            }
        },
        body,
    );
}
