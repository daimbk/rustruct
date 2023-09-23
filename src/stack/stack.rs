// Stack

struct Stack<T> {
    list: Vec<T>,
}

impl<T> Stack<T> {
    fn new(list: Vec<T>) -> Self {
        Stack { list: Vec::new() }
    }

    fn push(&mut self, data: T) {
        self.list.push(data);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
}
