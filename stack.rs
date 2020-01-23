pub struct Stack<T> {
    top : i32,
    store : Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {top : -1,store : Vec::new()}
    }

    pub fn push(&mut self,data : T) {
        self.top += 1;
        self.store.push(data);
    }

    pub fn pop(&mut self) -> &T {
        self.top -= 1;
        let x = &self.store[(self.top+1) as usize];
        return x;
    }
}

fn main() {
    println!("Hello");
    let mut s : Stack<i32> = Stack::new();
    s.push(10);
    s.push(20);
    println!("{}",s.pop());
}