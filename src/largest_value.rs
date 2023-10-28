pub fn largest_number<T: Ord>(list: &[T]) -> &T {
    let mut max = &list[0]; // O(1)

    for i in list {  // O(n)
        if i > max {
            max = i;
        }
    }

    max

}