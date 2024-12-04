pub(crate) struct WindowIterator<'a> {
    pub(crate) matrix: &'a Vec<Vec<u8>>,
    pub(crate) i: usize,
    pub(crate) j: usize,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl<'a> WindowIterator<'a> {
    pub(crate) fn new(matrix: &'a Vec<Vec<u8>>) -> Self {
        WindowIterator {
            matrix,
            i: 0,
            j: 0,
            rows: 3,
            cols: 3,
        }
    }
}

impl<'a> Iterator for WindowIterator<'a> {
    type Item = Vec<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we're out of bounds
        if self.i + self.rows > self.matrix.len() || self.j + self.cols > self.matrix[0].len() {
            return None;
        }

        // Create a new Vec<Vec<u8>> for the current 3x3 window
        let mut window = Vec::with_capacity(self.rows);

        for row in self.i..self.i + self.rows {
            window.push(self.matrix[row][self.j..self.j + self.cols].to_vec());
        }

        // Move to the next window (next column)
        if self.j + self.cols < self.matrix[0].len() {
            self.j += 1;
        } else {
            // If we reach the end of columns, move to the next row and reset columns
            self.j = 0;
            self.i += 1;
        }

        Some(window)
    }
}

#[test]
pub(crate) fn test() {
    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];

    let iterator = WindowIterator::new(&matrix);

    // Iterate over the 3x3 windows
    for window in iterator {
        for row in window {
            println!("{:?}", row);
        }
        println!("---");
    }
}
