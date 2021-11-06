use fixie::dispatch;
use fixie::router::Dispatch;

use log::debug;
use log::Level;
use std::cell::*;
use std::rc::Rc;
use sycamore::generic_node::*;
use sycamore::prelude::*;

#[derive(Debug, Clone)]
enum Events {
    InitializeDB,
    IncCounter,
    ResetForm,
}

fn main() {
    console_log::init_with_level(Level::Debug);

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = &document.body().unwrap();

    dispatch(Events::InitializeDB);
    dispatch(Events::TestA);
    dispatch(Events::TestB);
    dispatch(Events::TestC);

    render_plain(
        || {
            template! {
                p { "coca~pandas" }
            }
        },
        body,
    );
}
