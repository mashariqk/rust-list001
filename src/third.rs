use std::rc::Rc;

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
        Node { value, next: None }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            let x = node.next.as_ref().map(|node| &**node);
            self.next = x;
            let x = &node.value;
            x
        })
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            count: 0,
        }
    }

    fn len(&self) -> u32 {
        self.count
    }

    fn append(&self, value: T) -> List<T> {
        let new_count = self.count + 1;
        List {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
            count: new_count,
        }
    }

    fn tail(&self) -> List<T> {
        let mut new_count = 0;
        if self.count > 0 {
            new_count = self.count - 1;
        }
        List {
            head: self.head.as_ref().and_then(|node| {
                let x = node.next.clone();
                x
            }),
            count: new_count,
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            let x = &node.value;
            x
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| {
                let x = &**node;
                x
            }),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.len(), 3);

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        assert_eq!(list.len(), 2);

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.len(), 1);

        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
