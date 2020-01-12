use std::collections::HashMap;
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hash_tbl : HashMap<i32,i32>  = std::collections::HashMap::new();
    let mut largest_times : i32 = 0;
    let mut largest_val : i32 = 0;
    for e in nums {
        let mut current_elem_freq : i32 = 1;
        match hash_tbl.get(&e) {
            Some(times)  => {
                current_elem_freq = times+1;
                //hash_tbl.insert(e,times+1)
            },
            None => {

            }
        };

        hash_tbl.insert(e,current_elem_freq);
        if current_elem_freq > largest_times {
            largest_times = current_elem_freq;
            largest_val = e;
        }
    }
    return largest_val;
}

fn main() {
    let v = vec![3,2,3];
    println!("{}",majority_element(v));

}