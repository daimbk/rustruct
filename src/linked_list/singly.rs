// Singly Linked List

// <---- Node Structure ---->
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// <---- Linked List Structure ---->
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // pushes to front of list
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
}
