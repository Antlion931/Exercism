use super::Cell;
use std::cell::BorrowError;
use std::cell::RefCell;
use std::rc::Rc;
/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputCellId(pub usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComputeCellId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Counter {
    index: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self) -> usize {
        self.index += 1;
        self.index - 1
    }
}

pub fn try_get_values_to_vec<'a, T: Copy + PartialEq>(
    dependencies: impl AsRef<[Rc<RefCell<Cell<'a, T>>>]>,
    workhouse: &mut Vec<T>,
) -> Result<(), BorrowError> {
    workhouse.clear();
    for x in dependencies
        .as_ref()
        .iter()
        .map(|x| x.try_borrow().map(|x| x.get_value()))
    {
        workhouse.push(x?);
    }
    Ok(())
}
