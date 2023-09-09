//TODO:
//
// Refactor
// [x] make better benchmarks
// [x] why creating input and compute cells is so slow
//  - becouse they use vec and not hashmap
// [x] change Hashmap to vec and use InputId and ComputeId as indexes
//  - not good solution, need realocation in some point, changed to BTreeMap
// [ ] split in more files this 400 line monster 
use std::cell::RefCell;
use std::collections::{BTreeSet, BTreeMap};
use std::rc::Rc;
use std::rc::Weak;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputCellId {
    id: usize,
}
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
pub struct ComputeCellId {
    id: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId {
    id: usize,
}

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

enum Cell<'a, T> {
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
        match (self, other) {
            (Cell::Input(x), Cell::Input(y)) => x.partial_cmp(y),
            (Cell::Compute(x), Cell::Compute(y)) => x.partial_cmp(y),
            (Cell::Input(_), Cell::Compute(_)) => Some(std::cmp::Ordering::Less),
            (Cell::Compute(_), Cell::Input(_)) => Some(std::cmp::Ordering::Greater),
        }
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
    fn try_mut_input(&mut self) -> Option<&mut InputCell<'a, T>> {
        match self {
            Cell::Input(c) => Some(c),
            _ => None,
        }
    }

    fn try_input(&self) -> Option<&InputCell<'a, T>> {
        match self {
            Cell::Input(c) => Some(c),
            _ => None,
        }
    }

    fn try_mut_compute(&mut self) -> Option<&mut ComputeCell<'a, T>> {
        match self {
            Cell::Compute(c) => Some(c),
            _ => None,
        }
    }

    fn try_compute(&self) -> Option<&ComputeCell<'a, T>> {
        match self {
            Cell::Compute(c) => Some(c),
            _ => None,
        }
    }

    fn get_value(&self) -> T {
        match self {
            Cell::Input(s) => s.value,
            Cell::Compute(s) => s.value,
        }
    }

    fn add_to_update(&mut self, new: Weak<RefCell<Cell<'a, T>>>) {
        match self {
            Cell::Input(s) => s.to_update.push(new),
            Cell::Compute(s) => s.to_update.push(new),
        }
    }
}

struct ComputeCell<'a, T> {
    id: ComputeCellId,
    value: T,
    function: Box<dyn Fn(&[T]) -> T + 'a>,
    dependencies: Vec<Rc<RefCell<Cell<'a, T>>>>,
    to_update: Vec<Weak<RefCell<Cell<'a, T>>>>,
    callbacks: Vec<(CallbackId, Box<dyn FnMut(T) + 'a>)>,
}

impl<'a, T> PartialEq for ComputeCell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'a, T> Eq for ComputeCell<'a, T> {
    fn assert_receiver_is_total_eq(&self) {
        self.id.assert_receiver_is_total_eq()
    }
}

impl<'a, T> PartialOrd for ComputeCell<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<'a, T> Ord for ComputeCell<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    fn new<F: Fn(&[T]) -> T + 'a>(
        id: ComputeCellId,
        dependencies: &[Rc<RefCell<Cell<'a, T>>>],
        compute_func: F,
    ) -> Self {
        let value = compute_func(
            &dependencies
                .iter()
                .map(|x| x.borrow().get_value())
                .collect::<Vec<_>>(),
        );

        Self {
            id,
            function: Box::new(compute_func),
            value,
            dependencies: dependencies.into(),
            to_update: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    fn update(&mut self) {
        let new_value = (self.function)(
            &self
                .dependencies
                .iter()
                .map(|x| x.borrow().get_value())
                .collect::<Vec<_>>(),
        );

        if new_value != self.value {
            self.value = new_value;

            for c in self.callbacks.iter_mut() {
                (c.1)(self.value);
            }
        }
    }

    fn add_callback(&mut self, id: CallbackId, f: Box<dyn FnMut(T) + 'a>) {
        self.callbacks.push((id, f));
    }

    fn remove_callback(&mut self, id: CallbackId) -> Result<(), RemoveCallbackError> {
        let index = self
            .callbacks
            .iter()
            .position(|(i, _)| *i == id)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        let _ = self.callbacks.swap_remove(index);
        Ok(())
    }
}

struct InputCell<'a, T> {
    id: InputCellId,
    value: T,
    to_update: Vec<Weak<RefCell<Cell<'a, T>>>>,
}

impl<'a, T> PartialEq for InputCell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'a, T> Eq for InputCell<'a, T> {
    fn assert_receiver_is_total_eq(&self) {
        self.id.assert_receiver_is_total_eq()
    }
}

impl<'a, T> PartialOrd for InputCell<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<'a, T> Ord for InputCell<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'a, T> InputCell<'a, T> {
    fn set(&mut self, data: T) {
        self.value = data;
    }

    fn new(value: T, id: InputCellId) -> Self {
        Self {
            value,
            to_update: Vec::new(),
            id,
        }
    }
}

struct Counter {
    index: usize,
}

impl Counter {
    fn new() -> Self {
        Self { index: 0 }
    }

    fn next(&mut self) -> usize {
        self.index += 1;
        self.index - 1
    }
}

pub struct Reactor<'a, T> {
    counter_input: Counter,
    counter_compute: Counter,
    counter_callback: Counter,
    input_cells: BTreeMap<InputCellId, Rc<RefCell<Cell<'a, T>>>>,
    compute_cells: BTreeMap<ComputeCellId, Rc<RefCell<Cell<'a, T>>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + 'a> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            counter_input: Counter::new(),
            counter_callback: Counter::new(),
            counter_compute: Counter::new(),
            input_cells: BTreeMap::new(),
            compute_cells: BTreeMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId {
            id: self.counter_input.next(),
        };
        self.input_cells.insert(
            id,
            Rc::new(RefCell::new(Cell::Input(InputCell::new(initial, id)))),
        );

        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = ComputeCellId {
            id: self.counter_compute.next(),
        };
        let mut rc_dependencies = Vec::with_capacity(dependencies.len());

        for d in dependencies {
            match d {
                CellId::Input(id) => {
                    let cell = Rc::clone(self.input_cells.get(id).ok_or(*d)?);
                    rc_dependencies.push(cell);
                }
                CellId::Compute(id) => {
                    let cell = Rc::clone(self.compute_cells.get(id).ok_or(*d)?);
                    rc_dependencies.push(cell);
                }
            }
        }

        self.compute_cells.insert(
            id,
            Rc::new(RefCell::new(Cell::Compute(ComputeCell::new(
                id,
                &rc_dependencies,
                compute_func,
            )))),
        );

        let rc_function = Rc::clone(
            self.compute_cells
                .get(&id)
                .expect("was added in previous line"),
        );

        for d in rc_dependencies {
            d.borrow_mut().add_to_update(Rc::downgrade(&rc_function));
        }

        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(ref id) => self.input_cells.get(id).map(|x| x.borrow().get_value()),
            CellId::Compute(ref id) => self.compute_cells.get(id).map(|x| x.borrow().get_value()),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(cell) = self.input_cells.get(&id) {
            if let Some(s) = cell.borrow_mut().try_mut_input() {
                s.set(new_value);
            }
            if let Some(s) = cell.borrow().try_input() {
                let mut queue: BTreeSet<_> = s.to_update.iter().flat_map(|x| x.upgrade()).collect();

                while let Some(c) = queue.pop_first() {
                    if let Some(compute) = c.borrow_mut().try_mut_compute() {
                        compute.update();
                    }

                    if let Some(compute) = c.borrow().try_compute() {
                        for u in &compute.to_update {
                            if let Some(u) = u.upgrade() {
                                queue.insert(u);
                            }
                        }
                    }
                }
            }

            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let c_id = CallbackId {
            id: self.counter_callback.next(),
        };
        if let Some(c) = self.compute_cells.get(&id)?.borrow_mut().try_mut_compute() {
            c.add_callback(c_id, Box::new(callback));
        }
        Some(c_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(c) = self
            .compute_cells
            .get(&cell)
            .ok_or(RemoveCallbackError::NonexistentCell)?
            .borrow_mut()
            .try_mut_compute()
        {
            c.remove_callback(callback)
        } else {
            panic!("never happens");
        }
    }
}
