#[derive(Debug)]
pub struct List {
    head: Link,
    count: u32,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    value: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
            count: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn push(&mut self, value: i32) {
        let n = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(n));
        self.count += 1
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.count > 0 {
                self.count -= 1;
            }
            node.value
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        if self.len() > 0 {
            println!("About to destroy {:?}", self);
            let mut c_n = self.head.take();
            while let Some(mut bx) = c_n {
                c_n = bx.next.take();
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