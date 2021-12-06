// https://www.codewars.com/kata/5592e3bd57b64d00f3000047/

pub fn find_nb(m: u64) -> i32 {
    let mut sum: u64 = 0;
    let mut num = 0;
    for i in 1..m {
        sum += i.pow(3);
        num += 1;
        if sum == m {
            break;
        }
        
        if sum > m {
            println!("{} {}", sum, m);
            return -1;
        }
    }

    num
}
