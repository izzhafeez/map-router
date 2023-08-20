pub trait Writer<T> {
    fn write(s: T, path: &str) -> Result<(), ()>;
}