pub enum Colour {
    White,
    Yellow,
    Green,
}

impl From<char> for Colour {
    fn from(value: char) -> Self {
        match value {
            'g' => Colour::Green,
            'y' => Colour::Yellow,
            _ => Colour::White,
        }
    }
}
