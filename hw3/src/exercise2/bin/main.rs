use std::ops::Deref;
use std::cell::RefCell;
struct MyRc<T>{
  value:T,
  ref_count:RefCell<usize>
}
impl<T> MyRc<T>{
  fn new(value:T)->Self{
    MyRc{
      value,
      ref_count:RefCell::new(1)
    }
  }
  fn strong_count(&self)->usize{
    *self.ref_count.borrow()
  }
}
impl<T> Deref for MyRc<T>{
  type Target = T;
  fn deref(&self)->&T{
    &self.value
  }
}
impl<T:Clone> Clone for MyRc<T>{
  fn clone(&self)->MyRc<T>{
    *self.ref_count.borrow_mut() += 1;
    MyRc{
      value:self.value.clone(),
      ref_count:self.ref_count.clone()
    }
  }
}
impl<T> Drop for MyRc<T>{
  fn drop(&mut self){
    *self.ref_count.borrow_mut() -= 1;
    if *self.ref_count.borrow() == 0{
      println!("Dropping!");
    }
  }
}


fn main(){
  let five = MyRc::new(5);
  let five2 = MyRc::clone(&five);
  println!("{}", *five);
  println!("{}", MyRc::strong_count(&five2));
}
