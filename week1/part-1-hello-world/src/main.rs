fn sum(a: i32, b: i32) -> i32 {
   a + b
}

fn main() {
   let mut s: String = String::from("hello");
   s.push_str(" world");
   println!("{}", s);

   let mut v: Vec<i32> = Vec::new();
   v.push(2);
   v.push(5);

   for i in v.iter() {
       if (i % 2) == 0 {
           println!("{}", i);
       }
   }

   println!("{}", sum(v[0], v[1]));

}
