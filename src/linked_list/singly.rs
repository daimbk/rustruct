// Singly Linked List

// <---- Node Structure ---->
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

// <---- Linked List Structure ---->
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
        }
    }
}
