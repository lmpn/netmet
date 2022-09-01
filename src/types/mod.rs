use std::{collections::VecDeque, time::SystemTime};
pub type Metric = (SystemTime, u64);
pub type Transformation = Box<dyn Fn(&VecDeque<Metric>) -> f64 + Send + 'static>;
pub mod networkwatcher;
