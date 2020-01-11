//use std::io;

fn square(n : i32) -> i32 {
    return n*n;
}

fn printtill(n : i32) {
    let mut counter = 1;

    loop {
        println!("hellp world");
        if counter > n {
            break;
        }

        counter +=1;
    }
}

fn bubblesort(arr : &mut [i32]) {
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] > arr[j]  {
                arr.swap(j,i);
            }
        }
    }
}

fn main() {
    //println!("Hello, world!");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    // println!("You guessed {}",guess);

    // let mut x = 5;
    // x=6;
    // println!("{}",x);
    let guess : u32 = "42".parse().expect("Not a number");
    println!("{}",guess);

    let mut arr : [i32;5] = [1,200,30,40,5];
    println!("{}",arr[0]);

    let x : (i32,i32) = (10,9);
    println!("{}",x.0);

    println!("{}",square(99));

    printtill(10);

    bubblesort(&mut arr);

    //println!("{:?}",arr)
    for e in arr.iter() {
        println!("{},",e);
    }


}
