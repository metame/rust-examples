#![allow(dead_code)]
use std::boxed::Box;

pub fn new_box<T>(s: &[T]) -> Box<&[T]> {
    Box::new(s)
}

// impl basic recursive type
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_tests() {
        let s = vec![1,2,3,4,5];
        let br = new_box(&s[0..]);
        dbg!(br);
        let mut n1 = Node {val: 5, next: None};
        let n2 = Node {val: 10, next: None};
        n1.next = Some(Box::new(n2));
        let x = n1.next.unwrap().val;
        dbg!(x);
        assert!(false);
    }
}
