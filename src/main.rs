use my_rust_basics::*;


fn main() { 
   let arr = &["Gilbet", "Underat"];

   let s = binary_search(&"Gilbet", arr);

  let s = s.expect("The value is not found");
  let v = arr[s];

  println!("{v}");
}

