// https://www.codewars.com/kata/514b92a657cdc65150000006/

pub fn solution(num: i32) -> i32 {
    let mut sum = 0;

    let multiples = vec![3, 5];

    'outer: for i in 0..num {
        for j in &multiples {
            if i % j == 0 {
                sum += i;
                continue 'outer;
            }
        }
    }

    sum
}
