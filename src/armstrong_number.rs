pub fn is_armstrong_number(number: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut num: u32 = number;

    loop {
        digits.push(num % 10);

        num /= 10;
        if num == 0 {
            break;
        }
    }

    let sum_nth_power_of_digits: u32 = digits
        .iter()
        .map(|digit| digit.pow(digits.len() as u32))
        .sum();
    sum_nth_power_of_digits == number
}

pub fn get_armstrong_number(max_num: i32) {
   let mut a: i32;
   let mut b: i32;
   let mut c: i32;

   let mut v: i32;
   let mut q: i32;


    let mut count = 0;

    for i in 0..max_num {
        a = i;

        for j in 0..max_num {
            b = j;

            for k in 0..max_num {  
               
                c = k; 

                v = a*100 + b*10 + c; 
                q = a.pow(3) + b.pow(3) + c.pow(3); // c

                if v == q {
                    count += 1;

                    println!("Armstrong number {count}: {v}");
                }
            }
        }
    }
}
