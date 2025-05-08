#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::colour::Colour;

    #[test]
    fn test_colour_from_char() {
        assert_eq!(Colour::from('g'), Colour::Green);
        assert_eq!(Colour::from('y'), Colour::Yellow);
        assert_eq!(Colour::from('x'), Colour::White);
        assert_eq!(Colour::from('G'), Colour::White); // case-sensitive
        assert_eq!(Colour::from(' '), Colour::White);
    }
}
