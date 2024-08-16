#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum State {
    Unknown = -3,
    White = -2,
    Black = -1,
    Island(i32),
}

impl State {
    pub fn new(val: i32) -> Self {
        match val {
            ..=0 => State::Unknown,
            _ => State::Island(val),
        }
    }
}

impl From<State> for i32 {
   	fn from(val: State) -> Self {
        match val {
            State::Unknown => -3,
            State::White => -2,
            State::Black => -1,
            State::Island(x) => x,
        }
    }
}
