fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut index1 : i32 = m;
    let mut index2 : i32 = 0;
    while index2 < n {
        nums1[index1 as usize] = nums2[index2 as usize]; 
        index1 +=1;
        index2 +=1;
    }

    nums1.sort();
}

fn main() {
    let mut v1 = vec![1,2,3,0,0,0];
    let mut v2 = vec![2,5,6];
    merge(&mut v1,3,&mut v2,3);

    println!("{:?}",v1);
}