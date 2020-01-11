pub fn fib(n: i32) -> i32 {
 let mut vec = Vec::new();
 vec.push(0);
 vec.push(1);
 
 let mut index : i32 = 2;
 while index <= n {
     let current  = vec[(index-1) as usize] + vec[(index-2) as usize];
     vec.push(current);
     index += 1;
 }

 let f : usize = n as usize;
 return vec[f];
}

fn main() {
    println!("{}",fib(3));
}