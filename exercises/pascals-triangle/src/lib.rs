pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = vec![];
        for _ in 0..self.row_count {
            rows.push(rows.last().map_or(vec![1], Self::create_row));
        }
        rows
    }

    pub fn create_row(previous: &Vec<u32>) -> Vec<u32> {
        let slice = previous[1..previous.len()].to_vec();
        let zipped = previous.into_iter().zip(slice.into_iter());
        let mut result = zipped.fold(vec![1], |mut acc, i| {
            acc.push(i.0 + i.1);
            acc
        });

        result.push(1);
        result
    }
}
