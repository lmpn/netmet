pub trait Watcher {
    fn pool(&mut self);
    fn get_metrics(&self) -> Vec<(&(String, String), f64)>;
}
