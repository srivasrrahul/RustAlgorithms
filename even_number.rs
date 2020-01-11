
fn find_numbers(nums : Vec<i32>) -> i32 {
    let mut count : i32 = 0;
    for e in nums.iter() {
        let s = e.to_string();
        if s.len() % 2 == 0 {
            count += 1;
        }
    }

    return count;
    
}



fn main () {
    let vec = vec![12,345,2,6,7896];
    
    println!("{}",find_numbers(vec));
    
}