pub fn is_monotonic(a: Vec<i32>) -> bool {

    fn itr(x : Vec<i32>) -> bool {
        //assume vec has one element
        let mut last : i32 = x[0];
        
        let mut ret_value : bool = true;
        for i in 1..x.len() {
            if x[i] >= last {
                last = x[i];
            }else {
                ret_value = false;
                break;
            }
        }

        //println!("reached here {}",ret_value);

        
        if ret_value == false {
            ret_value = true;
            last = x[x.len()-1];
            for i in (0..(x.len()-1)).rev() {
                if x[i] >= last {
                    last = x[i];
                }else {
                    //println!("here {}",i);
                    return false;
                } 
            }
         }

        return ret_value;
    }

    // let mut y = a.clone();
    // y.reverse();
    return itr(a);

}

fn main () {
    let v = vec![6,5,4,4];
    println!("{}",is_monotonic(v));
}