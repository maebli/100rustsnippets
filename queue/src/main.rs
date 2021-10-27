use std::collections::VecDeque;

fn main() {

    let mut q:Queue<i32>=Queue::new();
    let e1:QueueEntry<i32> = QueueEntry{ data: 3 };

    assert!(q.is_empty());

    q.push_head(e1);
    assert_eq!(q.peek_head().unwrap().data,3);
    assert_eq!(q.pop_head().unwrap().data,3);

    let e1 = QueueEntry{ data: 4 };
    q.push_tail(e1);

    assert_eq!(q.peek_tail().unwrap().data,4);
    assert_eq!(q.pop_tail().unwrap().data,4);

    q.free();

    assert!(q.is_empty());

}

struct QueueEntry<T>{
    data:T,
}

struct Queue<T>{
    data:VecDeque<QueueEntry<T>>
}

impl <T> Queue<T> {

    fn free (&mut self){
        self.data.clear();
    }

    fn push_head(&mut self,entry:QueueEntry<T>){
        self.data.push_back(entry);
    }

    fn pop_head(&mut self) -> Option<QueueEntry<T>> {
        self.data.pop_back()
    }

    fn peek_head(&mut self) -> Option<&QueueEntry<T>> {
        self.data.back()
    }

    fn push_tail(&mut self,entry:QueueEntry<T>){
        self.data.push_front(entry);
    }

    fn pop_tail(&mut self) -> Option<QueueEntry<T>>{
        self.data.pop_front()
    }

    fn peek_tail(&mut self) -> Option<&QueueEntry<T>>{
        self.data.back()
    }

    fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }

    fn new() -> Queue<T>{
        Queue{
            data: Default::default()
        }
    }
}

