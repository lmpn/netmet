pub mod networkwatcherbuilder;
pub trait Builder<T: Default> {
    fn build(self) -> T;
}
