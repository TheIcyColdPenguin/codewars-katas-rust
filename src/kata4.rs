// https://www.codewars.com/kata/566fc12495810954b1000030/

pub fn nb_dig(n: i32, d: i32) -> i32 {
    let mut total = 0;

    for i in 0..=n {
        total += num_of_digits(i.pow(2), d);
    }

    total
}

fn num_of_digits(mut n: i32, d: i32) -> i32 {
    let mut sum = 0;

    loop {
        let last_digit = n % 10;
        n /= 10;

        if last_digit == d {
            sum += 1;
        }
        if n <= 0 {
            break;
        }
    }

    sum
}
