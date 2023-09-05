use std::ops::Add;
fn main(){
  let mut buf = Buffer::new();
  buf.buf.push(1);
  buf.buf.push(2);
  buf.buf.push(3);
  println!("{}", buf.sum());

}
struct Buffer<T> {
  buf: Vec<T>,
}
impl<T: Add<Output = T> + Copy + Default> Buffer<T> {
  fn new() -> Self {
    Buffer { buf: Vec::new() }
  }
  fn sum(&self) -> T {
    let mut sum = T::default();
    for i in &self.buf {
      sum = sum + *i;
    }
    sum
  }
}