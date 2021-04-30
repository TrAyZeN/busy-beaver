use crate::transition::Direction;
use std::collections::VecDeque;

/// A tape of binary-alphabet symbols
#[derive(Debug)]
pub struct Tape {
    /// Bit vector representing the tape
    cells: VecDeque<u64>,
    /// Position of the head on the tape
    head: isize,
    /// Range of the allocated tape representing the tape
    range: (isize, isize),
}

impl Tape {
    /// Creates a new tape with two cells and head at position 0.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            cells: vec![0, 0].into_iter().collect(),
            head: 0,
            range: (-64, 63),
        }
    }

    /// Reads the symbol on the tape at the head position.
    #[inline]
    pub fn read(&self) -> u8 {
        let (cell_index, bit_index) = self.get_cell_bit_index(self.head);

        ((self.cells[cell_index] >> bit_index) & 1) as u8
    }

    /// Writes the given binary symbol at the head position.
    ///
    /// # Panic
    /// Panics in `debug` mode if symbol is not 0 or 1.
    #[inline]
    pub fn write(&mut self, symbol: u8) {
        debug_assert!(symbol == 0 || symbol == 1);

        let (cell_index, bit_index) = self.get_cell_bit_index(self.head);

        if symbol == 1 {
            self.cells[cell_index] |= 1 << bit_index;
        } else {
            self.cells[cell_index] &= !(1 << bit_index);
        }
    }

    /// Moves the head one step in the given direction
    /// If the head moves out of the allocated cells range a new cell is allocated
    #[inline]
    pub fn move_head(&mut self, direction: Direction) {
        if self.head == self.range.0 {
            self.cells.push_front(0);
            self.range.0 -= 64;
        }

        if self.head == self.range.1 {
            self.cells.push_back(0);
            self.range.1 += 64;
        }

        self.head += direction as isize * 2 - 1;
    }

    /// Computes the cell index and bit index of the given position on the tape.
    ///
    /// # Panic
    /// Panics in `debug` mode if index is out of range.
    #[inline]
    fn get_cell_bit_index(&self, position: isize) -> (usize, u8) {
        debug_assert!(self.range.0 <= position && position <= self.range.1);

        let positive_position = (-self.range.0 + position) as usize;
        (positive_position >> 6, (positive_position & 63) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_head_left() {
        let mut t = Tape::new();
        assert_eq!(t.head, 0);

        t.move_head(Direction::Left);
        assert_eq!(t.head, -1);

        t.move_head(Direction::Left);
        assert_eq!(t.head, -2);
    }

    #[test]
    fn move_head_right() {
        let mut t = Tape::new();
        assert_eq!(t.head, 0);

        t.move_head(Direction::Right);
        assert_eq!(t.head, 1);

        t.move_head(Direction::Right);
        assert_eq!(t.head, 2);
    }

    #[test]
    fn move_head_both_directions() {
        let mut t = Tape::new();
        assert_eq!(t.head, 0);

        t.move_head(Direction::Right);
        assert_eq!(t.head, 1);

        t.move_head(Direction::Left);
        assert_eq!(t.head, 0);

        t.move_head(Direction::Left);
        assert_eq!(t.head, -1);

        t.move_head(Direction::Right);
        assert_eq!(t.head, 0);
    }

    #[test]
    fn move_head_left_overflow() {
        let mut t = Tape::new();
        for _ in 0..64 {
            t.move_head(Direction::Left);
        }

        assert_eq!(t.head, -64);
        assert_eq!(t.range, (-64, 63));

        t.move_head(Direction::Left);
        assert_eq!(t.head, -65);
        assert_eq!(t.range, (-128, 63));
    }

    #[test]
    fn move_head_right_overflow() {
        let mut t = Tape::new();
        for _ in 0..63 {
            t.move_head(Direction::Right);
        }

        assert_eq!(t.head, 63);
        assert_eq!(t.range, (-64, 63));

        t.move_head(Direction::Right);
        assert_eq!(t.head, 64);
        assert_eq!(t.range, (-64, 127));
    }

    #[test]
    fn read() {
        let t = Tape::new();

        assert_eq!(t.read(), 0);
    }

    #[test]
    fn write_0() {
        let mut t = Tape::new();

        t.write(0);
        assert_eq!(t.read(), 0);
    }

    #[test]
    fn write_1() {
        let mut t = Tape::new();

        t.write(1);
        assert_eq!(t.read(), 1);
    }

    #[test]
    fn cell_bit_index() {
        let mut t = Tape::new();

        assert_eq!(t.get_cell_bit_index(0), (1, 0));
        assert_eq!(t.get_cell_bit_index(1), (1, 1));
        assert_eq!(t.get_cell_bit_index(-1), (0, 63));

        t.range.0 = -128;
        assert_eq!(t.get_cell_bit_index(0), (2, 0));
        assert_eq!(t.get_cell_bit_index(1), (2, 1));
        assert_eq!(t.get_cell_bit_index(-1), (1, 63));
    }
}
