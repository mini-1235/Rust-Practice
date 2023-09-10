use std::cell::RefCell;
#[derive(Debug)]
struct SimpleStack<T>{
  //refcell is a wrapper type that allows mutable borrows checked at runtime
  stack:RefCell<Vec<T>>
}
impl <T> SimpleStack<T>{
  fn new()->Self{
    SimpleStack{
      stack:RefCell::new(Vec::new())
    }
  }
  fn push(&self, item:T){
    self.stack.borrow_mut().push(item);
  }
  fn pop(&self)->Option<T>{
    self.stack.borrow_mut().pop()
  }
}
fn main(){
  let stack = SimpleStack::new();
  stack.push(1);
  stack.push(2);
  stack.push(3);
 println!("Popped {:?}", stack.pop());
  println!("Popped {:?}", stack.pop());
  stack.push(4);
  println!("Popped {:?}", stack.pop());
  println!("Popped {:?}", stack.pop());
  println!("Popped {:?}", stack.pop());
}
