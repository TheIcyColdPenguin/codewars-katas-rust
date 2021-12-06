// https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut new_data = vec![];

    for i in data {
        new_data.push(if i.0 >= 55 && i.1 > 7 {
            "Senior".to_owned()
        } else {
            "Open".to_owned()
        })
    }
    new_data
}
