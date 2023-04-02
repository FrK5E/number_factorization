use std::collections::HashMap;

pub type Factorization = HashMap<u32, u32>;

pub fn get_one() -> Factorization {
    return HashMap::new();
}

pub fn get_factorization(k: u32) -> Factorization {
    let mut fact = HashMap::new();

    let mut k1 = k;
    let mut i = 2;

    while k1 >= 2 {
        if k1 % i == 0 {
            k1 = k1 / i;
            if fact.contains_key(&i) {
                *fact.get_mut(&i).expect("Panic!!!!") += 1;
            } else {
                fact.insert(i, 1);
            }
            i = 2;
        } else {
            i += 1;
        }
    }

    return fact;
}

pub fn multiply(a: Factorization, b: Factorization) -> Factorization {
    let mut fact = a;

    for i in b.keys() {
        if fact.contains_key(i) {
            *fact.get_mut(&i).expect("panic!") += b[i];
        } else {
            fact.insert(*i, b[i]);
        }
    }
    return fact;
}

pub fn division(a: Factorization, b: Factorization) -> Factorization {
    let mut fact = a;
    for i in b.keys() {
        *fact.get_mut(&i).expect("panic!") -= b[i];
    }

    fact
}

pub fn get_number(a: Factorization) -> u128 {
    let mut result = 1u128;
    for i in a.keys() {
        let k2 = i.pow(a[i]);
        result = result * k2 as u128;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = get_factorization(4);
        let benchmark = [(2, 2)].iter().cloned().collect();
        assert_eq!(result, benchmark);
    }

    #[test]
    fn test2() {
        let result = get_factorization(20);
        let benchmark = [(2, 2), (5, 1)].iter().cloned().collect();
        assert_eq!(result, benchmark);
    }

    #[test]
    fn test3() {
        let a = get_factorization(20);
        let b = get_factorization(20);
        let c = get_factorization(20 * 20);
        let result = multiply(a, b);
        let benchmark = [(2, 4), (5, 2)].iter().cloned().collect();
        assert_eq!(result, c);
        assert_eq!(result, benchmark);
    }

    #[test]
    fn test4() {
        let a = get_factorization(20);
        let b = get_factorization(10);
        let c = division(a, b);
        let benchmark = [(2, 1), (5, 0)].iter().cloned().collect();
        assert_eq!(c, benchmark);
    }

    #[test]
    fn test5() {
        let k = 20;
        let a = get_factorization(k);
        let b = get_number(a);
        assert_eq!(k, b as u32);
    }

    #[test]
    fn test6() {
        let one = get_one();
        let k = get_number(one);
        assert_eq!(1, k as u32);
    }
}
