# rust-fsm-traffic-light

「最小・過不足なし」を徹底したRust版ステートマシン（信号機）

A minimal example of a **Finite State Machine (FSM)** implemented in Rust,  
using a traffic light as the simplest possible model.

This repository is intended for **learning and experiencing** how state machines
work in actual code — not for building a real traffic control system.

---

## What is this?

This project demonstrates a **finite state machine** with three states:

```
RED → GREEN → YELLOW → RED
```

The goal is **not** the traffic light itself, but to understand:

- What a “state” really is
- How state transitions can be expressed in code
- Why explicitly modeling states is safer than using `if` statements

---

## Why Rust?

This FSM can be written in many languages (PHP, Python, Ruby, etc.).

However, Rust provides a unique learning experience:

- States are defined as a **closed set** using `enum`
- All state transitions must be **exhaustively handled**
- Invalid or missing transitions cause **compile-time errors**

In other words:

> If the design is wrong, the program does not compile.

This makes Rust ideal for *experiencing* why state machines are useful.

---

## Project Structure

```
.
├── Cargo.toml
├── Dockerfile
└── src
    └── main.rs
```

- No external dependencies
- No frameworks
- Only core Rust features

---

## The Core Idea (FSM)

```rust
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
```

Key points:

- States are explicit
- Transitions are centralized
- Impossible states cannot exist

## Run with Docker (Recommended)

You do not need Rust installed locally.

## Build

```bash
docker build -t rust-fsm .
```

## Run

```bash
docker run --rm rust-fsm
```

## Output

```bash
current state: Red
current state: Green
current state: Yellow
current state: Red
current state: Green
current state: Yellow
```

## Try This (Highly Recommended)

Add a new state:

```rust
enum State {
    Red,
    Green,
    Yellow,
    Blink, // new state
}
```

Rust will immediately report a compile error:

```text
non-exhaustive patterns
```

This is the core benefit of using a state machine with a strong type system.

## Who is this for?

- People who have heard of “state machines” but never really understood them
- Developers tired of complex `if / else` logic
- Anyone curious about Rust’s approach to correctness
- Readers coming from a blog or article about FSMs

---

## Related Article

This repository accompanies a blog article explaining:

- Why state machines matter
- Why simple examples are enough
- How Rust makes invalid states unrepresentable

(Article link will be added here)

## License

MIT License
