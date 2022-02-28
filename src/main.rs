/*
Enabling WASM:
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve - starts a webserver to test the WASM on
*/

use seed::{prelude::*, *};

#[derive(Debug)]
struct Model {
    // button will contain a number so a number needs to be stored
    counter: i32,
    history: Vec<i32>,
}

enum Msg {
    Increment,
    Decrement,
    Reset,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 0,
        history: Vec::new(),
    }
}

// equivalent of the controller
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter = std::cmp::max(0, model.counter - 1),
        Msg::Reset => {
            model.history.push(model.counter);
            model.counter = 0;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div!["This is the current count: ", model.counter],
        "This is the Increment button: ",
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
        div![
            "\nThis is the Decrement button: ",
            button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        ],
        div![button!["Reset? ", ev(Ev::Click, |_| Msg::Reset),]],
        div!["Count history: ", &model.history]
    ]
}

fn main() {
    App::start("app", init, update, view);
}
