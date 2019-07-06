use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
    count: u32,
}

#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
            count: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn push(&mut self, value: i32) {
        let n = Node {
            value,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(n));
        self.count += 1
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(mut s) => {
                self.head = mem::replace(&mut s.next, Link::Empty);
                if self.count > 0 {
                    self.count -= 1;
                }
                Some(s.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        if self.len() > 0 {
            println!("About to destroy {:?}", self);
            let mut c_n = mem::replace(&mut self.head, Link::Empty);
            while let Link::More(mut bx) = c_n {
                c_n = mem::replace(&mut bx.next, Link::Empty);
                println!("About to destroy {:?}", c_n);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_for_new_list_count() {
        let mylist1 = List::new();
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
}
