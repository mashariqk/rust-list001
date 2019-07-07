use std::mem;
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // NEW!
    count: u32,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
            count: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            if self.count > 0 {
                self.count -= 1;
            }
            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(0, list.len());

        // Check empty list behaves right
        assert_eq!(list.pop(), None);
        assert_eq!(0, list.len());

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(3, list.len());

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(1, list.len());

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);
        assert_eq!(3, list.len());

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(1, list.len());

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(0, list.len());
        assert_eq!(list.pop(), None);
        assert_eq!(0, list.len());

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);
        assert_eq!(2, list.len());

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(1, list.len());
        assert_eq!(list.pop(), Some(7));
        assert_eq!(0, list.len());
        assert_eq!(list.pop(), None);
        assert_eq!(0, list.len());
    }
}
