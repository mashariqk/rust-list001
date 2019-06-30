use std::mem;

#[derive(Debug)]
pub struct List{
    head:Link
}

#[derive(Debug)]
pub enum Link{
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
pub struct Node{
    value:i32,
    next:Link
}

impl List{
    pub fn new() -> Self{
        List{
            head:Link::Empty
        }
    }

    pub fn push(&mut self,value:i32){
        let n = Node{
            value,
            next:mem::replace(&mut self.head,Link::Empty)
        };
        self.head = Link::More(Box::new(n));
    }

    pub fn pop(&mut self) -> Option<Node>{
        match mem::replace(&mut self.head,Link::Empty) {
            Link::Empty => None,
            Link::More(mut s) => {
                self.head = mem::replace(&mut s.next,Link::Empty);
                Some(*s)
            }
        }
    }
}