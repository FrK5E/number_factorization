use std::collections::HashMap;


type Factorization = HashMap<u32,u32>;


fn get_factorization( k: u32) -> Factorization { 
    
    let mut fact = HashMap::new();

    let mut k1 = k; 
    let mut i = 2; 

    while k1>=2 { 
        if k1%i == 0 { 
            k1 = k1 / i;
            if fact.contains_key(&i) { 
                *fact.get_mut(&i).expect("Panic!!!!") += 1;  
            } else { 
                fact.insert(i, 1); 
            }
            i=2;
        } else { 
            i+=1; 
        }
    }

    return fact;

}

//pub fn get_factorization_of_factorial_shortened(hi: u32, low: u32) -> Factorization {
//    for i in low..hi+1 { 
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = get_factorization(4);
        let benchmark  = [(2, 2)].iter().cloned().collect(); 
        assert_eq!(result, benchmark );
    }

    #[test]
    fn test2() {
        let result = get_factorization(20);
        let benchmark  = [(2, 2), (5,1)].iter().cloned().collect(); 
        assert_eq!(result, benchmark );
    }
}
