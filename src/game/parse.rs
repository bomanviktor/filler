use std::io;
#[allow(dead_code)]
pub struct GameInfo {
    board: Vec<String>,
    pieces: [char; 2],
    start_location: [(u64, u64); 2],
}

pub fn board() -> String {
    instruction()
}

pub fn pieces() -> [char; 2] {
    ['a', 'b']
}

pub fn start_location() -> [(u64, u64); 2] {
    [(0, 0), (1, 1)]
}

pub fn instruction() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("INSTRUCTION: {buffer}");
    buffer
}
