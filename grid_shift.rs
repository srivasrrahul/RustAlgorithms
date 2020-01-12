

fn next(current_row : i32,current_col : i32,grid_row : i32,grid_col : i32) -> (i32,i32) {

    let mut ret_col : i32 = current_col;
    let mut ret_row : i32 = current_row;
    let mut if_next_row = false;

    if current_col == grid_col {
        ret_col = 0;
        if_next_row = true;
        
    }else {
        ret_col = current_col + 1;
    }

    if if_next_row == true {
        if current_row == grid_row {
            ret_row = 0;
        }else {
            ret_row = current_row + 1;
        }
    }

    return (ret_row,ret_col);
}
pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut response = grid.clone();
    let total : i32 = grid.len() as i32 * grid[0].len() as i32;
    let shift : i32 = k % total;      
    let rows : i32 = grid.len() as i32 - 1; //From 0
    let cols : i32 = grid[0].len() as i32 - 1; //From 0

    let total : i32 = (rows+1)*(cols+1);

    let mut source_row : i32 = 0;
    let mut source_col : i32 = 0;

    let mut dest_row : i32 = 0;
    let mut dest_col : i32 = 0;

    let mut next_till : i32 = 0;

    //println!("Shift is {}",shift);
    while next_till < shift {
        let d1 = next(dest_row,dest_col,rows,cols);
        dest_row = d1.0;
        dest_col = d1.1;
        next_till +=1;
        
    }   

    //println!("Dest row,col is {},{}",dest_row,dest_col);
    

    let mut begin : i32 = 0;

    
    while begin < total {

        println!("=====");
        println!("Source row,col is {},{}",source_row,source_col);
        println!("Dest row,col is {},{}",dest_row,dest_col);


        response[dest_row as usize][dest_col as usize] = grid[source_row as usize][source_col as usize];
        let d1 = next(dest_row,dest_col,rows,cols);
        let s1 = next(source_row,source_col,rows,cols);
        dest_row = d1.0;
        dest_col = d1.1;
        source_row = s1.0;
        source_col = s1.1;

        println!("Updated Source row,col is {},{}",source_row,source_col);
        println!("Updated Dest row,col is {},{}",dest_row,dest_col);
        begin += 1;
        
    }

    return response; 

}

fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    let v3 = vec![7,8,9];

    let mut v : Vec<Vec<i32>> = Vec::new();
    v.push(v1);
    v.push(v2);
    v.push(v3);

    println!("{:?}",v);

    let res = shift_grid(v.clone(),2);
    println!("{:?}",v);
    println!("{:?}",res);
    
}