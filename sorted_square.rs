fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut x : Vec<i32> = a.into_iter().map(|x| x*x).collect();
    x.sort();
    return x;

}

fn main() {
    let v = vec![-4,-1,0,3,10];
    let v1 = sorted_squares(v);
    println!("{:?}",v1);
}