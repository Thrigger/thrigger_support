pub mod index_of;

/// Finds a pattern between the two specified strings
///
/// Returns the first occurens of a string between the two specified patterns
///
/// # Example
/// '''
/// assert_eq!(find_between("<a<b>>","<",">"), "a<b");
/// '''
pub fn find_between<'a>(string: &'a str, pattern_start: &str, pattern_stop: &str) -> Option<&'a str> {
    let mut start = -1;
    let mut stop = -1;

    for (i, each) in string.chars().enumerate() {
        let each_as_string = each.to_string();
        let each_as_str = each_as_string.as_str();
        if start == -1 && each_as_str == pattern_start {
            start = i as isize;
        } else if start != -1 && each_as_str == pattern_stop {
            stop = i as isize;
            break;
        }
    }

    if stop == -1 || stop - start <= 1 {
        return None;
    } 
    Some(&string[(start+1) as usize..stop as usize])
}

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

/// Greatest Common Divisor
///
/// Finds the greatest common divisior of two numbers using euclid's algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut bigger = a;
    let mut smaller = b;
    if a < b {
        bigger = b;
        smaller = a;
    }

    while smaller>0 {
        let rest = bigger % smaller;
        bigger = smaller;
        smaller = rest;
    }

    bigger
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
    fn test_find_between() {
        assert_eq!(find_between("(1)", "(", ")"), Some("1"));
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(45, 75), 15);
        assert_eq!(gcd(876, 204), 12);
        assert_eq!(gcd(876, 240), 12);
        assert_eq!(gcd(11319, 9943), 1);
    }
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

