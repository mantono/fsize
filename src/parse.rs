use crate::ParseError;

pub(crate) fn parse(input: Vec<u8>) -> Result<Parts, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }
    let mut i = Parts::new();
    for n in input.into_iter() {
        i.add(n)?
    }
    Ok(i)
}

pub(crate) struct Parts {
    num: u64,
    unit: Option<char>,
}

impl Parts {
    pub fn new() -> Self {
        Parts { num: 0, unit: None }
    }

    pub fn num(&self) -> u64 {
        self.num
    }

    pub fn unit(&self) -> Option<char> {
        self.unit
    }

    pub fn add(&mut self, byte: u8) -> Result<(), ParseError> {
        if self.unit.is_some() {
            return Err(ParseError::MultiChar);
        }

        if byte.is_ascii_digit() {
            self.num *= 10;
            self.num += (byte - 48) as u64;
        } else if byte.is_ascii_alphabetic() {
            let c: char = char::try_from(byte).expect("Alpha");
            self.unit = Some(c);
        } else {
            return Err(ParseError::InvalidByte(byte));
        };
        Ok(())
    }
}
