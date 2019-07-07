use core::mem;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    count: u32,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

impl<'a, T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            count: 0,
        }
    }

    fn len(&self) -> u32 {
        self.count
    }

    pub fn push_front(&mut self, elem: T) {
        self.count += 1;
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                                            // total: +2 new_head, +0 old_head -- OK!
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
                                            // total: +2 new_head -- OK!
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.count > 0 {
            self.count = self.count - 1;
        }
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                },
            }
            let x = Rc::try_unwrap(old_head).ok().unwrap().into_inner().value;
            x
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.len(),0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(),0);

        // Populate list
        list.push_front(1);
        assert_eq!(list.len(),1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(),3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(),1);

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.len(),3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.len(),1);

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(),0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(),0);
    }
}

