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
        let mylist1:List<i32> = List::new();
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