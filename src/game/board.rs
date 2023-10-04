#[derive(Default, Debug)]
pub struct Board {
    pub width: u64,
    pub height: u64,
    pub pieces: Vec<String>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            pieces: Vec::new(),
        }
    }
}
