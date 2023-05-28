use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Clone)]
struct BucketState {
    capacity_first: u8,
    capacity_second: u8,
    first: u8,
    second: u8,
    moves: u8,
}

impl PartialEq for BucketState {
    fn eq(&self, other: &Self) -> bool {
        self.capacity_first == other.capacity_first
            && self.capacity_second == other.capacity_second
            && self.second == other.second
            && self.first == other.first
    }
}

impl BucketState {
    fn make_moves(&self) -> Vec<BucketState> {
        let mut result = Vec::with_capacity(4);

        // Fill
        result.push(BucketState {
            first: self.capacity_first,
            moves: self.moves + 1,
            ..*self
        });
        result.push(BucketState {
            second: self.capacity_second,
            moves: self.moves + 1,
            ..*self
        });

        // Empty
        result.push(BucketState {
            first: 0,
            moves: self.moves + 1,
            ..*self
        });
        result.push(BucketState {
            second: 0,
            moves: self.moves + 1,
            ..*self
        });

        // Pour
        let diff_first = (self.capacity_first - self.first).min(self.second);
        let diff_second = (self.capacity_second - self.second).min(self.first);

        result.push(BucketState {
            first: self.first + diff_first,
            second: self.second - diff_first,
            moves: self.moves + 1,
            ..*self
        });
        result.push(BucketState {
            first: self.first - diff_second,
            second: self.second + diff_second,
            moves: self.moves + 1,
            ..*self
        });

        result
    }

    fn is_winning(&self, target: u8) -> Option<BucketStats> {
        if self.first == target {
            return Some(BucketStats {
                moves: self.moves,
                goal_bucket: Bucket::One,
                other_bucket: self.second,
            });
        } else if self.second == target {
            return Some(BucketStats {
                moves: self.moves,
                goal_bucket: Bucket::Two,
                other_bucket: self.first,
            });
        }

        None
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut in_queue = Vec::new();
    let mut queue = VecDeque::new();

    let invalid = match start_bucket {
        Bucket::One => BucketState {
            capacity_first: capacity_1,
            capacity_second: capacity_2,
            first: 0,
            second: capacity_2,
            moves: 1,
        },
        Bucket::Two => BucketState {
            capacity_first: capacity_1,
            capacity_second: capacity_2,
            first: capacity_1,
            second: 0,
            moves: 1,
        },
    };

    let start_bucket = match start_bucket {
        Bucket::One => BucketState {
            capacity_first: capacity_1,
            capacity_second: capacity_2,
            first: capacity_1,
            second: 0,
            moves: 1,
        },
        Bucket::Two => BucketState {
            capacity_first: capacity_1,
            capacity_second: capacity_2,
            first: 0,
            second: capacity_2,
            moves: 1,
        },
    };

    in_queue.push(start_bucket.clone());
    queue.push_back(start_bucket);

    while let Some(b) = queue.pop_front() {
        if let Some(result) = b.is_winning(goal) {
            return Some(result);
        }

        for new in b.make_moves() {
            if !in_queue.contains(&new) && new != invalid {
                in_queue.push(new.clone());
                queue.push_back(new);
            }
        }
    }

    None
}
