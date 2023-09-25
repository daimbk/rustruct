// Stack

struct Stack<T: Clone> {
    list: Vec<T>,
}

impl<T: Clone> Stack<T> {
    fn new(list: Vec<T>) -> Self {
        Stack { list: Vec::new() }
    }

    fn push(&mut self, data: T) {
        self.list.push(data);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    fn top(&mut self) -> Option<T> {
        self.list.last().cloned()
    }

    fn is_empty(&mut self) -> bool {
        let top = self.top();

        match top {
            Some(val) => false,
            None => true,
        }
    }
}
