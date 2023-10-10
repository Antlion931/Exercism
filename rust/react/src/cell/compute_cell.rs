use super::Cell;
use crate::common::*;
use std::cell::BorrowError;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub struct ComputeCell<'a, T> {
    id: ComputeCellId,
    pub value: T,
    function: Box<dyn Fn(&[T]) -> T + 'a>,
    dependencies: Vec<Rc<RefCell<Cell<'a, T>>>>,
    pub to_update: Vec<Weak<RefCell<Cell<'a, T>>>>,
    callbacks: Vec<(CallbackId, Box<dyn FnMut(T) + 'a>)>,
    workhouse: Vec<T>,
}

impl<'a, T> PartialEq for ComputeCell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'a, T> Eq for ComputeCell<'a, T> {}

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
    pub fn new<F: Fn(&[T]) -> T + 'a>(
        id: ComputeCellId,
        dependencies: &[Rc<RefCell<Cell<'a, T>>>],
        compute_func: F,
    ) -> Result<Self, BorrowError> {
        let mut workhouse = Vec::new();
        try_get_values_to_vec(dependencies, &mut workhouse)?;

        let value = compute_func(&workhouse);

        Ok(Self {
            id,
            function: Box::new(compute_func),
            value,
            dependencies: dependencies.into(),
            to_update: Vec::new(),
            callbacks: Vec::new(),
            workhouse,
        })
    }

    fn compute(&mut self) -> Result<T, BorrowError> {
        try_get_values_to_vec(&self.dependencies, &mut self.workhouse)?;
        Ok((self.function)(&self.workhouse))
    }

    pub fn update(&mut self) -> Result<(), BorrowError> {
        let new_value = self.compute()?;

        if new_value != self.value {
            self.value = new_value;

            for c in self.callbacks.iter_mut() {
                (c.1)(self.value);
            }
        }

        Ok(())
    }

    pub fn add_callback(&mut self, id: CallbackId, f: Box<dyn FnMut(T) + 'a>) {
        self.callbacks.push((id, f));
    }

    pub fn remove_callback(&mut self, id: CallbackId) -> Result<(), RemoveCallbackError> {
        let index = self
            .callbacks
            .iter()
            .position(|(i, _)| *i == id)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        let _ = self.callbacks.swap_remove(index);
        Ok(())
    }
}
