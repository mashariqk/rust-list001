use std::rc::Rc;
use core::borrow::BorrowMut;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    count: u32,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None,
        }
    }
}

impl<T> List<T> {

    fn new() -> Self {
        List {
            head: None,
            count: 0,
        }
    }

    fn len(&self) -> u32{
        self.count
    }

    fn append(&mut self, value: T) -> List<T> {
        let new_count = self.count + 1;
        List {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
            count: new_count,
        }
    }

    fn tail(&mut self) -> List<T> {
        let mut new_count = 0;
        if self.count > 0 {
            new_count = self.count - 1;
        }
        List {
            head: self.head.as_ref().and_then(|node| {
                node.next.clone()
            }),
            count: new_count,
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(),0);

        let mut list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.len(),3);

        let mut list = list.tail();
        assert_eq!(list.head(), Some(&2));
        assert_eq!(list.len(),2);

        let mut list = list.tail();
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.len(),1);

        let mut list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(),0);

        // Make sure empty tail works
        let mut list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(),0);
    }
}