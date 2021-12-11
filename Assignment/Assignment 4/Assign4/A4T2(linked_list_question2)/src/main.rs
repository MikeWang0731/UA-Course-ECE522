use self::LinkedList::*;
use im::list::*;
use std::borrow::Borrow;

#[derive(Debug, PartialEq)]

pub enum LinkedList<T> {
    Tail,
    Head(List<T>),
}

impl<T> LinkedList<T> {
    // empty
    pub fn empty() -> Self {
        LinkedList::Tail
    }
    // new
    pub fn new(t: T) -> Self {
        LinkedList::Head(cons(t, List::new()))
    }

    // push
    pub fn push(self, t: T) -> Self {
        match self {
            LinkedList::Tail => LinkedList::new(t),
            LinkedList::Head(list) => LinkedList::Head(cons(t, list)),
        }
    }

    //push_back
    pub fn push_back(&mut self, t: T) {
        match self {
            LinkedList::Tail => *self = LinkedList::Head(cons(t, List::new())),
            LinkedList::Head(ref mut list) => *list = list.push_back(t),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l, Head(cons(4, cons(3, List::new()))));
        l.push_back(2);
        assert_eq!(l, Head(cons(4, cons(3, cons(2, List::new())))));
    }
}
