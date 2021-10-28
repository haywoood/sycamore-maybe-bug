use sycamore::prelude::*;
use sycamore::context::{ContextProvider, ContextProviderProps, use_context};
use web_sys::Event;

#[derive(Clone)]
struct AppState {
    count: Signal<i32>,
    counts: Signal<Vec<Signal<i32>>>,
}

#[component(Count<G>)]
fn count(c: Signal<i32>) -> Template<G> {
    let state = use_context::<AppState>();
    let handle_click = cloned!((state) => move |_event: Event| {
        let new_count: Vec<Signal<i32>> = state.counts
            .get()
            .as_ref()
            .clone()
            .into_iter()
            .chain(Some(Signal::new(0)))
            .collect();
        state.counts.set(new_count)
    });

    let handle_click_two = cloned!((state) => move |_event: Event| {
        state.count.set(state.count.get().as_ref() + 1)
    });

    template! {
        p { (c.get()) }
        p { (state.count.get()) }

        // This will fail with `Uncaught RuntimeError: unreachable executed`
        button(on:click=handle_click) {
            "click me"
        }

        // this works
        button(on:click=handle_click_two) {
            "click me 2"
        }
    }
}

#[component(App<G>)]
fn app() -> Template<G> {
    let state = use_context::<AppState>();
    template! {
        h1 {"count"}
        Indexed(IndexedProps {
            iterable: state.counts.handle(),
            template: move |count| template! {
                Count(count)
            }
        })
    }

}

fn main() {
    let app_state = AppState {
        count: Signal::new(10),
        counts: Signal::new(vec![Signal::new(0)]),
    };

    sycamore::render(|| template! {
        ContextProvider(ContextProviderProps {
            value: app_state,
            children: || template! {
                div {
                    App()
                }
            }
        })
    });
}
