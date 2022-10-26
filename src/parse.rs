use crate::ParseError;

pub(crate) fn parse(input: Vec<u8>) -> Result<State, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }

    input
        .into_iter()
        .try_fold(State::new(), |state, byte| state.add(byte))
}

pub(crate) enum State {
    Num(u64),
    NumUnit(u64, char),
}

impl State {
    fn new() -> State {
        State::Num(0)
    }

    pub fn num(&self) -> u64 {
        match self {
            State::Num(n) => *n,
            State::NumUnit(n, _) => *n,
        }
    }

    pub fn unit(&self) -> Option<char> {
        match self {
            State::Num(_) => None,
            State::NumUnit(_, unit) => Some(*unit),
        }
    }

    fn add(self, byte: u8) -> Result<State, ParseError> {
        let value: AsciiChar = byte.try_into().map_err(|_| ParseError::InvalidByte(byte))?;

        match self {
            State::Num(s) => match value {
                AsciiChar::Digit(n) => Ok(Self::Num((s * 10) + (n as u64))),
                AsciiChar::Alpha(u) => Ok(Self::NumUnit(s, u)),
            },
            State::NumUnit(_, _) => Err(ParseError::MultiChar),
        }
    }
}

enum AsciiChar {
    Digit(u8),
    Alpha(char),
}

impl TryFrom<u8> for AsciiChar {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(AsciiChar::Digit(value - 48))
        } else if value.is_ascii_alphabetic() {
            let value: char = char::try_from(value).expect("Is ASCII alpha");
            Ok(AsciiChar::Alpha(value))
        } else {
            Err(())
        }
    }
}
