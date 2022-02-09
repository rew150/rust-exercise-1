#[cfg(test)]
mod test {
    use crate::vector::Vector;

    #[test]
    fn new_vec() {
        let vec: Vector<i32> = Vector::new();
        assert_eq!(0, vec.size);
        assert_eq!(0, vec.cap);
    }

    #[test]
    fn vec_of_str_ref() {
        let string = String::from("scope string");
        let mut vec: Vector<&str> = Vector::new();
        vec.push(&string);
        vec.push("Static string");
        assert_eq!(2, vec.size);
        assert_eq!(2, vec.cap);
        assert_eq!(Some("Static string"), vec.pop());
        assert_eq!(Some("scope string"), vec.pop());
        assert_eq!(0, vec.size);
        assert_eq!(2, vec.cap);
    }

    #[test]
    fn test_cap_exponential_grow() {
        let cap_order: [usize; 10] = [1,2,4,4,8,8,8,8,16,16];

        let mut vec: Vector<usize> = Vector::new();
        for i in 0..10 {
            vec.push(i);
            assert_eq!(cap_order[i], vec.cap);
            assert_eq!(i+1, vec.size);
        }
        for i in (0..10).rev() {
            let val = vec.pop();
            assert_eq!(Some(i), val);
        }
        assert_eq!(0, vec.size);
        assert_eq!(16, vec.cap);
    }
    
    #[test]
    fn iterator() {
        let mut vec: Vector<usize> = Vector::new();
        for i in 0..10 {
            vec.push(i);
        }
        for (i, item) in vec.into_iter().enumerate() {
            assert_eq!(i, item);
        }
    }

    #[test]
    fn back_iterator() {
        let mut vec: Vector<usize> = Vector::new();
        for i in 0..10 {
            vec.push(i);
        }
        for (i, item) in vec.into_iter().rev().enumerate() {
            assert_eq!(9-i, item);
        }
    }

    #[test]
    fn sized() {
        let mut vec: Vector<String> = Vector::new();
        for i in 0..10 {
            vec.push(i.to_string());
        }
        for (i, item) in vec.into_iter().enumerate() {
            assert_eq!(String::from(i.to_string()), item);
        }
    }
}