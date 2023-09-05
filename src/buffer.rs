pub struct Buffer<T> {
    pub x: Vec<T>,
}
impl<T> Buffer<T>
where
    T: std::ops::Add<Output = T> + Clone + Default,
{
    //sum函数
    pub fn sum(&self) -> T {
        let mut sum = T::default();
        for i in &self.x {
            sum = sum + i.clone();
        }
        sum
    }
}
