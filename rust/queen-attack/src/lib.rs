#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }

        Some(Self { rank, file }) 
    }

    fn on_diagonal_line(&self, other: &Self) -> bool {
        self.rank - self.file == other.rank - other.file || self.rank + self.file == other.rank + other.file
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank || self.position.file == other.position.file || self.position.on_diagonal_line(&other.position)
    }
}
