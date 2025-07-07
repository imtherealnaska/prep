use std::collections::HashSet;

fn happy_number(n: i32) -> bool {
    fn sum_of_sq(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            let digit = num % 10;
            sum += digit * digit;
            num /= 10;
        }
        sum
    }

    let mut current = n;
    let mut set = HashSet::new();

    while current != 1 && !set.contains(&current) {
        set.insert(current);
        current = sum_of_sq(current);
    }

    current == 1
}
