#![allow(dead_code)]
// /*
//     Последовательностью Фибоначчи называется последовательность чисел,
//     которая удовлетворяет следующим условиям:
//     - элемент последовательности с индексом 0 - число 0
//     - элемент с индексом 1 - число 1
//     - каждый последующий элемент равен сумме двух предыдущих.

//     0, 1, 1, 2, 3, 5, 8, 13, 21 ...

//     Написать функцию, которая вычислит элемент последовательности с индексом n.

//     * Написать вторую функцию, которая вернёт последовательность Фибонначи
//       от первого элемента до n-ого. Написать тесты
// */
fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut a = 1;
    let mut b = 1;

    for _ in 2..n {
        let c = a + b;

        a = b;
        b = c;
    }

    b
}

#[allow(unused_variables)]
pub fn fib_until(from: u32, to: u32) -> Vec<u32> {
    if to <= 1 {
        return vec![to];
    }

    let mut fib: Vec<u32> = vec![1, 1];

    let mut a = 1;
    let mut b = 1;

    for _ in 2..to {
        let c = a + b;

        fib.push(c);

        a = b;
        b = c;
    }

    let mut acc: Vec<u32> = Vec::new();

    for (index, value) in fib.iter().enumerate() {
        if (from as usize) <= index + 1 {
            acc.push(*value)
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(7), 13);

        assert_eq!(fib_until(0, 0), vec![0]);
        assert_eq!(fib_until(1, 0), vec![0]);
        assert_eq!(fib_until(0, 1), vec![1]);
        assert_eq!(fib_until(1, 1), vec![1]);
        assert_eq!(fib_until(0, 7), vec![1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(fib_until(5, 7), vec![5, 8, 13]);
        assert_eq!(fib_until(6, 6), vec![8]);
    }
}
