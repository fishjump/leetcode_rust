pub trait UnorderedSame<T> {
    fn unordered_same(&self, x: T) -> bool;
}

impl UnorderedSame<Vec<i32>> for Vec<i32> {
    fn unordered_same(&self, x: Vec<i32>) -> bool {
        let (mut a, mut b) = (self.clone(), x.clone());
        a.sort();
        b.sort();

        a == b
    }
}
