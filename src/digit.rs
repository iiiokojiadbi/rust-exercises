#![allow(dead_code)]
// /*
//     Написать функцию, которая будет вычислять произведение цифр числа,
//     при это цифра 0 игнорируется. Затем повторить операцию с результатом
//     произведения, пока не получится число, состоящее из одной цифры.
// */
fn accumulate_digits(num: u32) -> u32 {
    let mut acc = if num == 0 {
        return num;
    } else {
        1
    };

    for digit in num.to_string().chars() {
        if let Some(number) = digit.to_digit(10) {
            if number > 0 {
                acc *= number
            }
        }
    }

    acc
}

pub fn digit_product(mut n: u32) -> u8 {
    loop {
        n = accumulate_digits(n);

        if n < 10 {
            return n as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
    }
}
