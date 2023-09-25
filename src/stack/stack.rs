// Stack

pub struct Stack<T: Clone> {
    list: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { list: Vec::new() }
    }

    pub fn push(&mut self, data: T) {
        self.list.push(data);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn top(&mut self) -> Option<T> {
        self.list.last().cloned()
    }

    pub fn is_empty(&mut self) -> bool {
        let top = self.top();

        match top {
            Some(_val) => false,
            None => true,
        }
    }
}
