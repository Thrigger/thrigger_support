pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point {x,y}
    }
    pub fn new_origin() -> Point {
        Point::new(0,0)
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        String::from(format!("{},{}", self.x, self.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let p1 = Point::new(1,5);
        let mut p2 = Point::new(1,5);
        let p3 = Point::new_origin();

        assert_eq!("1,5",p1.to_string());
        assert_eq!("1,5",p2.to_string());
        assert_eq!("0,0",p3.to_string());

        p2.x += 1;
        assert_eq!("2,5",p2.to_string());
    }
}

