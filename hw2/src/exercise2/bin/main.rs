fn main(){
  let s1 = "hello";
  let s2 = "world";
  compare_string(s1, s2);
  println!("{} > {} ? {}", s1, s2, compare_string(s1, s2));
  println!("{} > {} ? {}", s2, s1, compare_string(s2, s1));
}
//compare string by dictionary order, return true if s1>s2, otherwise false
fn compare_string(s1: &str, s2: &str) -> bool{
    let mut s1 = s1.chars();
    let mut s2 = s2.chars();
    loop{
        let c1 = s1.next();
        let c2 = s2.next();
        match (c1, c2){
            (None, None) => return false,
            (None, Some(_)) => return false,
            (Some(_), None) => return true,
            (Some(c1), Some(c2)) => {
                if c1 > c2{
                    return true;
                }
                else if c1 < c2{
                    return false;
                }
            }
        }
    }
}