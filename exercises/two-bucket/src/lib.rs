use std::cmp;

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

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    unimplemented!(
        "Given one bucket of capacity {}, another of capacity {}, starting with {:?}, find pours to reach {}",
        capacity_1,
        capacity_2,
        start_bucket,
        goal,
    );
}

struct BucketState {
    capacity: u8,
    content: u8,
}

impl BucketState {
    fn empty(&mut self) {
        self.content = 0
    }

    fn fill(&mut self) {
        self.content = self.capacity
    }

    fn freeSpace(&self) -> u8 {
        self.capacity - self.content
    }

    fn pour(&mut self, other: &mut BucketState) {
        let to_fill = std::cmp::min(self.freeSpace(), other.content);
        self.content += to_fill;
        other.content -= to_fill;
    }
}
