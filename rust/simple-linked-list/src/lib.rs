use std::fmt::Debug;
use std::iter::FromIterator;

pub struct SimpleLinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T: Debug> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        self.size += 1;
        let node = Node { data: element, next: self.head.take() };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.size -= 1; // have to be after ? to not substract 1 from 0
        self.head = node.next;
        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        let node = self.head.as_ref()?;
        Some(&node.data)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();

        while let Some(value) = self.pop() {
            reversed.push(value);
        }
        reversed
    }
}

impl<T: Debug> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = Self::new();

        for value in iter {
            result.push(value);
        }
        
        result
    }
}

impl<T: Debug> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();

        while let Some(value) = linked_list.pop() {
            result.push(value);
        }

        result.reverse();
        result
    }
}
