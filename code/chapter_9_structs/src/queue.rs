#[derive(Debug)]
pub struct Queue<T> {
    pub older: Vec<T>,
    pub younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(older: Vec<T>, newer: Vec<T>) -> Queue<T> {
        Queue::<T> {
            older: older,
            younger: newer,
        }
    }

    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
        }
        // Bring the elements in younger over to older, and put them in the promised order
        use std::mem::swap;
        swap(&mut self.older, &mut self.younger);
        self.older.reverse();

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option
        self.older.pop()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl Queue<f64> {
    pub fn sum(&self) -> f64 {
        self.older.iter().sum::<f64>() + self.younger.iter().sum::<f64>()
    }
}
