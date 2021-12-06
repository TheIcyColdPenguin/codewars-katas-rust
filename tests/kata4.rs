use codewars::kata4::*;

fn dotest(n: i32, d: i32, exp: i32) -> () {
    println!("n: {:?}", n);
    println!("d: {:?}", d);

    let ans = nb_dig(n, d);

    println!("actual:\n{:?}", ans);
    println!("expect:\n{:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
}

#[test]
fn basic_tests() {
    dotest(550, 5, 213);
    dotest(5750, 0, 4700);
}
