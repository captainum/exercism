#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            None
        } else {
            Some(Self(rank, file))
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = self.position.0.abs_diff(other.position.0);
        let dy = self.position.1.abs_diff(other.position.1);

        dx == 0 ||
        dy == 0 ||
        dx == dy
    }
}