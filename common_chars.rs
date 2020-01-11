
pub fn common_chars(a: Vec<String>) -> Vec<String> {
    let mut index : [i32;26] = [0;26];
    let mut count = 0;
    for e in a.iter() {
        let mut local_index : [i32;26] = [0;26];
        
        for c in e.chars() {
            
            let v : usize = c as usize - 'a' as usize; 
            //index[v] += 1;
            local_index[v] += 1;
           
        }

        //println!("L {:?}",local_index);

        //Normalize index
        for i in 0..26 {
            if count > 0 {
                index[i] = std::cmp::min(index[i],local_index[i]); 
            }else {
                index[i] = local_index[i];
                
            }


        }

        count +=1;

        //println!("G {:?}",index);
    }        

    //println!("{:?}",index);
    let mut res :Vec<String> = Vec::new();
    for i in 0..26 {
        // if index[i as usize] >= a.len() as i32 {
        //     let mut times = 0;
        //     let max_time = index[i as usize]/a.len() as i32;
        //     while times < max_time {
        //         let c = 'a' as u8 + i;
        //         let s = c as char;
        //         res.push(String::from(s.to_string()));
        //         times += 1;
        //     }

        // }
        let mut times = 0;
        while times < index[i as usize] {
            let c = 'a' as u8 + i;
            let s = c as char;
            res.push(String::from(s.to_string()));
            times += 1;

        }
    }

    return res;
}

fn main() {
    //["acabcddd","bcbdbcbd","baddbadb","cbdddcac","aacbcccd","ccccddda","cababaab","addcaccd"]
    let v : Vec<String> = vec![String::from("bella"),String::from("label"),String::from("roller")];
    let v1 = common_chars(v);
    println!("{:?}",v1);


}