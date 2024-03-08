trait HasSize {
    fn size(&self) -> usize;
}

impl HasSize for &String {
    fn size(&self) -> usize {
        self.len()
    }
}

impl HasSize for i32 {
    fn size(&self) -> usize {
        *self as usize
    }
}

fn my_push<T>(vector: &mut Vec<T>, element: T)
where
    T: HasSize
{
    println!("The size of the new element is {}.", element.size());
    vector.push(element);
}

fn main() {
    let my_string = String::from("foo");
    let mut my_vec = Vec::<&String>::new();
    my_push(&mut my_vec, &my_string);
    // drop(my_string);
    println!("{:?}", my_vec);

    let mut int_vec = vec![1, 2, 3];
    my_push(&mut int_vec, 7);
    println!("{:?}", int_vec);
}

// fn my_str_push<'a>(vector: &mut Vec<&'a String>, element: &'a String) {
//     vector.push(element);
// }

// fn my_int_push(vector: &mut Vec<i32>, element: i32) {
//     vector.push(element);
// }

/*
 * ArrayList<Object> arrayList;
 * 
 * arrayList.push(element);
 * 
 * 
 */
