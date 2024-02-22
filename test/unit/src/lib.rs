struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn can_not_hold() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 50, height: 100 };
        assert!(!rect1.can_hold(&rect2));
    }

    #[test]
    fn equality_check() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn non_equality_check() {
        assert_ne!(2 + 2, 5);
    }

    #[test]
    #[should_panic]
    fn asserting_panic() {
        panic!("This test will fail");
    }

    #[test]
    #[should_panic(expected = "This test will fail")]
    fn assert_panic_with_message() {
       if true {
           panic!("This test will fail");
       }
        panic!("This test will fail also");
    }
}