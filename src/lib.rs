pub mod kind;

#[cfg(test)]
mod tests {
    use super::*;
    use kind::kinds;

    #[test]
    fn kinds() {
        fn generic<K: kind::Kind>(xs: K::F<i32>) -> i32
        where
            K::F<i32>: IntoIterator<Item = i32>,
            K::F<i32>: Sized,
        {
            xs.into_iter().sum()
        }

        let sum = generic::<kinds::std::vec::Vec>(vec![1, 2, 3, 4, 5]);
        assert_eq!(sum, 15);
    }
}
