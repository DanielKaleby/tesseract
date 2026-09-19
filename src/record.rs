use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub enum Cube {
    Two,
    #[default]
    Three,
    Four,
    Five,
    Six,
    Seven,
    // TODO: add all the other WCA events
}
impl Cube {
    pub fn as_string(&self) -> String {
        match self {
            Cube::Two => "2x2x2".to_string(),
            Cube::Three => "3x3x3".to_string(),
            Cube::Four => "4x4x4".to_string(),
            Cube::Five => "5x5x5".to_string(),
            Cube::Six => "6x6x6".to_string(),
            Cube::Seven => "7x7x7".to_string(),
        }
    }
    pub fn config_key(&self) -> &str {
        match self {
            Cube::Two => "record_two",
            Cube::Three => "record_three",
            Cube::Four => "record_four",
            Cube::Five => "record_five",
            Cube::Six => "record_six",
            Cube::Seven => "record_seven",
        }
    }
}

