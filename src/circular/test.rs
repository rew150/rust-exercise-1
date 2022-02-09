#[cfg(test)]
mod test {

    use crate::circular::Circular;

    #[test]
    fn test_iter() {
        let mut c: Circular<String> = Circular::new();
        for i in 1..=3 {
            c.prepend(i.to_string());
        }
        for (i, val) in c.iter_mut().take(6).enumerate() {
            val.push_str(&i.to_string());
        }
        assert_eq!(3, c.size);
        assert_eq!("303",c[0]);
        assert_eq!("214",c[1]);
        assert_eq!("125",c[2]);
    }
}