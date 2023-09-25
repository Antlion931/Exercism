use std::cell::RefCell;
use std::rc::Weak;
use crate::common::*;
use super::Cell;

pub struct InputCell<'a, T> {
    id: InputCellId,
    pub value: T,
    pub to_update: Vec<Weak<RefCell<Cell<'a, T>>>>,
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
    pub fn set(&mut self, data: T) {
        self.value = data;
    }

    pub fn new(value: T, id: InputCellId) -> Self {
        Self {
            value,
            to_update: Vec::new(),
            id,
        }
    }
}


