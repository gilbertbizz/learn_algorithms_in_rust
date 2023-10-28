pub fn array_search<T: Ord + Copy>(list: &[T], num: T) {
    let mut n: T;
    
    for i in 0..list.len() {
        if list[i] == num {
            let r = &list[i];
            n = *r;

        } else {
            panic!("Not found!")
        } 
    }
    
}

    
    
