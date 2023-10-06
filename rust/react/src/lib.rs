//TODO:
//
// Refactor
// [x] make better benchmarks
// [x] why creating input and compute cells is so slow
//  - becouse they use vec and not hashmap
// [x] change Hashmap to vec and use InputId and ComputeId as indexes
//  - not good solution, need realocation in some point, changed to BTreeMap
// [x] split in more files this 400 line monster 
// [x] change all ids to tuples
// [x] rethink visibility
// [ ] simplify code as much as you can
//  - [x] avoid nest forest in set_value method
//  - [ ] add comment explaining why borrow and mut borrow work there
//  - [ ] try to find a way to change mut borrow to borrow
//  - [x] change all method that are not in lib.rs and borrow input and compute to return Result
// [ ] add workhous vec
//  - [x] add it at least to every compute
//   - success, alocation decresed from 2,244 allocs and 909,341 bytes to 1,304 allocs and 462,589 bytes speeding by 20,000 ns/iter in benchmark for this test
//  - [ ] add it to Reactor common for every compute
mod cell;
mod common;
use std::cell::RefCell;
use std::collections::{BTreeSet, BTreeMap};
use std::rc::Rc;
pub use common::*;
use cell::*;

pub struct Reactor<'a, T> {
    counter_input: Counter,
    counter_compute: Counter,
    counter_callback: Counter,
    input_cells: BTreeMap<InputCellId, Rc<RefCell<Cell<'a, T>>>>,
    compute_cells: BTreeMap<ComputeCellId, Rc<RefCell<Cell<'a, T>>>>,
}

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
        let id = InputCellId(self.counter_input.next());

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
        let id = ComputeCellId(self.counter_compute.next());

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
            ).expect("There is no borrow at this point")))),
        );

        let rc_function = Rc::clone(
            self.compute_cells
                .get(&id)
                .expect("was added in previous line"),
        );

        for d in rc_dependencies {
            d.try_borrow_mut().expect("Borrows from creating new compute should be droped").add_to_update(Rc::downgrade(&rc_function));
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
        if !self.input_cells.contains_key(&id) {
            return false;
        }

        let cell = &self.input_cells[&id]; 

        cell.borrow_mut().try_mut_input().expect("There are only input cells in input_cells").set(new_value);

        let mut queue: BTreeSet<_> = cell.borrow().try_input().expect("There are only input cells in input_cells").to_update.iter().flat_map(|x| x.upgrade()).collect();

        while let Some(c) = queue.pop_first() {
            c.borrow_mut().try_mut_compute().expect("There are only compute cells in queue, as in to_update").update();

            for u in &c.borrow().try_compute().expect("There are only compute cells in queue, as in to_update").to_update {
                if let Some(u) = u.upgrade() {
                    queue.insert(u);
                }
            }
        }

        true
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
        let c_id = CallbackId(self.counter_callback.next());

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
