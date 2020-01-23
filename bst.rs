struct Node<T> {
    //data : i32,
    data : T,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>
}

impl<T : Ord> Node<T> {
    pub fn new(v : T) -> Self {
        return Node {data : v,left : None,right : None}
    }

    pub fn add(&mut self,v : T)  {
        if v < self.data {
            match &mut self.left {
                Some(left_tree) => {
                    left_tree.add(v);
                },
                None => {
                    let new_node = Box::new(Node::new(v));
                    self.left = Some(new_node);
                    //return new_node;
                }
            }
        }else {
            match &mut self.right {
                Some(right_tree) => right_tree.add(v),
                None => {
                    let new_node = Box::new(Node::new(v));
                    self.right = Some(new_node);
                    //return new_node;
                }
            }
        }
    }

    fn visit(&self,f : fn(&T)->())   {
        match &self.left {
            Some(left) => {
                left.visit(f);
            },
            None => {

            }
        }

        //println!("{}",self.data);
        f(&self.data);
        match &self.right {
            Some(rtree) => {
                rtree.visit(f);
            },
            None => {

            }
        }

    }
}


// impl Node {
//     pub fn new(v : i32) -> Self {
//         return Node {data : v,left : None,right : None}
//     }

//     pub fn add(&mut self,v : i32)  {
//         if v < self.data {
//             match &mut self.left {
//                 Some(left_tree) => {
//                     left_tree.add(v);
//                 },
//                 None => {
//                     let new_node = Box::new(Node::new(v));
//                     self.left = Some(new_node);
//                     //return new_node;
//                 }
//             }
//         }else {
//             match &mut self.right {
//                 Some(right_tree) => right_tree.add(v),
//                 None => {
//                     let new_node = Box::new(Node::new(v));
//                     self.right = Some(new_node);
//                     //return new_node;
//                 }
//             }
//         }
//     }

//     fn visit(&self,f : fn(i32)->())   {
//         match &self.left {
//             Some(left) => {
//                 left.visit(f);
//             },
//             None => {

//             }
//         }

//         //println!("{}",self.data);
//         f(self.data);
//         match &self.right {
//             Some(rtree) => {
//                 rtree.visit(f);
//             },
//             None => {

//             }
//         }

//     }
// }



fn main() {
    println!("hello world");
    let mut root = Node::new(20);
    root.add(10);
    root.add(30);
    root.add(5);
    root.add(21);
    root.add(31);
    root.add(70);
    root.add(35);
    let print = |n : &i32| -> () {
        println!("{}",n);
    };
    root.visit(print);

    let mut r1 = Node::new("hello");
    r1.add("pello");
    r1.add("gello");
    r1.add("rahul");
    r1.add("shloak");
    r1.add("asdhjh");
    let print_string =|n : &&str| -> () {
        println!("{}",n);
    };

    r1.visit(print_string);
}