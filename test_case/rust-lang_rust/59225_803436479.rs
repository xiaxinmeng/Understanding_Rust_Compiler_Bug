
enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(State::color),
    Echo(String),
    Move(Point),
    Quit,

}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

