#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    count: u32,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            count: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn push(&mut self, value: T) {
        let n = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(n));
        self.count += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.count > 0 {
                self.count -= 1;
            }
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        if self.len() > 0 {
            let mut c_n = self.head.take();
            while let Some(mut bx) = c_n {
                c_n = bx.next.take();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_for_new_list_count() {
        let mylist1: List<i32> = List::new();
        assert_eq!(mylist1.len(), 0);
    }

    #[test]
    fn test_for_push_pop_and_length() {
        let mut mylist2 = List::new();

        assert_eq!(mylist2.len(), 0);

        mylist2.push(3);
        mylist2.push(39);
        mylist2.push(7);
        mylist2.push(43);

        assert_eq!(mylist2.len(), 4);

        assert_eq!(mylist2.pop(), Some(43));

        assert_eq!(mylist2.pop(), Some(7));

        assert_eq!(mylist2.len(), 2);

        mylist2.pop();
        mylist2.pop();

        assert_eq!(mylist2.pop(), None);

        assert_eq!(mylist2.len(), 0);
    }

    #[test]
    fn peek() {
        let mut mylist3: List<i32> = List::new();
        mylist3.push(3);
        mylist3.push(9);
        mylist3.push(138);
        mylist3.push(6);
        assert_eq!(mylist3.peek(), Some(&6));
    }

    #[test]
    fn peek_mut() {
        let mut mylist4: List<i32> = List::new();
        mylist4.push(3);
        mylist4.push(9);
        mylist4.push(138);
        mylist4.push(6);
        assert_eq!(mylist4.peek(), Some(&6));
        mylist4.peek_mut().map(|vale| *vale = 97);
        assert_eq!(mylist4.peek(), Some(&97));
        assert_eq!(mylist4.pop(), Some(97));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        //        println!("List is now {:?}",list); // Wont compile since it has moved
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
