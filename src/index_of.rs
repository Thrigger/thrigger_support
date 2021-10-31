
trait IndexOf<T> {
    fn index_of(&self, element: T) -> Option<usize>;
    fn index_of_reversed(&self, element: T) -> Option<usize>;
}

impl<T: std::cmp::PartialEq> IndexOf<T> for [T] {
    fn index_of(&self, element: T) -> Option<usize> {
        for (i, each) in self.iter().enumerate() {
            if *each == element {
                return Some(i);
            }
        }
        None
    }
    fn index_of_reversed(&self, element: T) -> Option<usize> {
        for (i, each) in self.iter().rev().enumerate() {
            if *each == element {
                return Some(self.len() - i - 1);
            }
        }
        None
    }
}

impl<T: std::cmp::PartialEq> IndexOf<T> for Vec<T> {
    fn index_of(&self, element: T) -> Option<usize> {
        for (i, each) in self.iter().enumerate() {
            if *each == element {
                return Some(i);
            }
        }
        None
    }
    fn index_of_reversed(&self, element: T) -> Option<usize> {
        for (i, each) in self.iter().rev().enumerate() {
            if *each == element {
                return Some(self.len() - i - 1);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        let a = vec![1, 2, 3, 4, 5, 5, 4, 3, 2];
        let b = vec!["a", "abc", "ABC", "A"];
        let c = vec!['a', 'b', 'c', 'a'];
        assert_eq!(a.index_of(3), Some(2));
        assert_eq!(a.index_of(6), None);
        assert_eq!(a.index_of_reversed(3), Some(7));
        assert_eq!(a[..a.len()-1].index_of_reversed(2), Some(1));
        assert_eq!(b.index_of("A"), Some(3));
        assert_eq!(b.index_of("Abc"), None);
        assert_eq!(b.index_of_reversed("a"), Some(0));
        assert_eq!(b[..b.len()-1].index_of_reversed("A"), None);
        assert_eq!(c.index_of('a'), Some(0));
        assert_eq!(c.index_of('1'), None);
        assert_eq!(c[2..].index_of('b'), None);
        assert_eq!(c.index_of_reversed('a'), Some(3));
        assert_eq!(c[..c.len()-1].index_of_reversed('a'), Some(0));
    }
}

