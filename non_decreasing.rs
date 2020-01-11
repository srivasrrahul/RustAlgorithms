fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
    let mut x = a.clone();
     x.sort_by_key(|k| k % 2);
     return x;   
}

fn main() {
    let vec = vec![3,1,2,4];
    let v1 = sort_array_by_parity(vec);
    println!("{:?}",v1);
}