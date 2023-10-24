mod compute_cell;
mod input_cell;

pub use compute_cell::ComputeCell;
pub use input_cell::InputCell;
use std::cell::RefCell;
use std::rc::Weak;

pub enum Cell<'a, T> {
    Input(InputCell<'a, T>),
    Compute(ComputeCell<'a, T>),
}

impl<'a, T> PartialEq for Cell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cell::Input(x), Cell::Input(y)) => x.eq(y),
            (Cell::Compute(x), Cell::Compute(y)) => x.eq(y),
            _ => false,
        }
    }
}

impl<'a, T> Eq for Cell<'a, T> {}

impl<'a, T> PartialOrd for Cell<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a, T> Ord for Cell<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Cell::Input(x), Cell::Input(y)) => x.cmp(y),
            (Cell::Compute(x), Cell::Compute(y)) => x.cmp(y),
            (Cell::Input(_), Cell::Compute(_)) => std::cmp::Ordering::Less,
            (Cell::Compute(_), Cell::Input(_)) => std::cmp::Ordering::Greater,
        }
    }
}

impl<'a, T: Copy> Cell<'a, T> {
    pub fn try_mut_input(&mut self) -> Option<&mut InputCell<'a, T>> {
        match self {
            Cell::Input(c) => Some(c),
            _ => None,
        }
    }

    pub fn try_input(&self) -> Option<&InputCell<'a, T>> {
        match self {
            Cell::Input(c) => Some(c),
            _ => None,
        }
    }

    pub fn try_mut_compute(&mut self) -> Option<&mut ComputeCell<'a, T>> {
        match self {
            Cell::Compute(c) => Some(c),
            _ => None,
        }
    }

    pub fn try_compute(&self) -> Option<&ComputeCell<'a, T>> {
        match self {
            Cell::Compute(c) => Some(c),
            _ => None,
        }
    }

    pub fn get_value(&self) -> T {
        match self {
            Cell::Input(s) => s.value,
            Cell::Compute(s) => s.value,
        }
    }

    pub fn add_to_update(&mut self, new: Weak<RefCell<Cell<'a, T>>>) {
        match self {
            Cell::Input(s) => s.to_update.push(new),
            Cell::Compute(s) => s.to_update.push(new),
        }
    }
}
