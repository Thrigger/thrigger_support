/// Least Common Multiple
///
/// This function finds the least common multiple of a Vec<i64>
pub fn lcm(input: &Vec<i64>) -> i64 {
    let biggest_value = find_max(input) as usize;
    let mut i = biggest_value;
    
    let output = loop {
        let mut possible_lcm = true;
        for each in input {
            if i as i64 % *each != 0 {
                possible_lcm = false;
                break;
            }
        }
        if possible_lcm {
            break i;
        }
        
        i += biggest_value;
    };
    output as i64
}

/// Find biggest value in vector
///
/// This function finds the biggest value in the vector
pub fn find_max(input: &Vec<i64>) -> i64 {
    let mut largest = 0;
    
    for each in input {
        if *each > largest {
            largest = *each;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&vec![10, 30, 60]), 60);
        assert_eq!(lcm(&vec![11, 30, 60]), 660);
        assert_eq!(lcm(&vec![2, 65]), 130);
        assert_eq!(lcm(&vec![100, 30, 60]), 300);
    }
    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&vec![10, 30, 60]), 60);
        assert_eq!(find_max(&vec![11, 30, 60]), 60);
        assert_eq!(find_max(&vec![2, 65]), 65);
        assert_eq!(find_max(&vec![100, 30, 60]), 100);
    }
}
