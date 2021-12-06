// https://www.codewars.com/kata/551b4501ac0447318f0009cd/

pub fn boolean_to_string(b: bool) -> String {
    (if b { "true" } else { "false" }).to_owned()
}
