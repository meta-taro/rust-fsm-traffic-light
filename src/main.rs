#[derive(Debug)]
enum State {
    Red,
    Green,
    Yellow,
}

fn next(state: State) -> State {
    match state {
        State::Red => State::Green,
        State::Green => State::Yellow,
        State::Yellow => State::Red,
    }
}

fn main() {
    let mut state = State::Red;

    for _ in 0..6 {
        println!("current state: {:?}", state);
        state = next(state);
    }
}
