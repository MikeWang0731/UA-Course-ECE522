#[derive(Debug, PartialEq)]

pub enum LinkedList<T> {
	Tail,
	Head(T, Box<LinkedList<T>>),
}

use self::LinkedList::*;

impl<T> LinkedList<T> {
	// empty
	pub fn empty() -> Self {
		LinkedList::Tail
	}

	// new
	pub fn new(t: T) -> Self {
		LinkedList::Head(t, Box::new(Tail))
	}

	// Push
	pub fn push(self, t: T) -> Self {
		LinkedList::Head(t, Box::new(self))
	}

	// push_back
	pub fn push_back(&mut self, t: T) {
		match self {
			LinkedList::Tail => {
				*self = LinkedList::new(t);
			}
			LinkedList::Head(_value, ref mut next) => {
				next.push_back(t);
			}
		}
	}
}

fn main() {
	let empty = LinkedList::empty(1);
	println!("{:?}", empty);

	let mut ll = LinkedList::new(3);
	println!("{:?}", ll);

	ll = ll.push(2);
	println!("{:?}", ll);

	ll.push_back(4);
	println!("{:?}", ll);
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn it_works(){
		
		let mut l = LinkedList::new(3);

		l= l.push(4);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Tail)))));

		l.push_back(2);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Head(2,Box::new(Tail)))))));
	}
}

// Output:
// Tail
// Head(3, Tail)
// Head(2, Head(3, Tail))
// Head(2, Head(3, Head(4, Tail)))
