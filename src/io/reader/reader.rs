pub trait Reader<T> {
    fn read(string: &str) -> T;
}