use std::{collections::HashMap, collections::VecDeque, time::SystemTime};

use sysinfo::{NetworkExt, RefreshKind, System, SystemExt};
use ubyte::ByteUnit;

use crate::types::{Metric, Transformation};
use crate::watcher::Watcher;

pub struct NetworkWatcher {
    system: System,
    inames: Vec<String>,
    watch_rx: bool,
    watch_tx: bool,
    unit: ByteUnit,
    transformations: Vec<Transformation>,
    metrics: HashMap<(String, String), VecDeque<Metric>>,
}

impl NetworkWatcher {
    pub fn inames(&mut self, arg: Vec<String>) {
        self.inames = arg;
    }

    pub fn watch_rx(&mut self, arg: bool) {
        self.watch_rx = arg;
    }

    pub fn watch_tx(&mut self, arg: bool) {
        self.watch_tx = arg;
    }

    pub fn unit(&mut self, arg: ByteUnit) {
        self.unit = arg;
    }
    pub fn transformations(&mut self, arg: Vec<Transformation>) {
        self.transformations = arg;
    }
}

impl Default for NetworkWatcher {
    fn default() -> Self {
        Self {
            system: System::new_with_specifics(RefreshKind::new().with_networks()),
            inames: Default::default(),
            watch_rx: Default::default(),
            watch_tx: Default::default(),
            unit: Default::default(),
            transformations: Default::default(),
            metrics: Default::default(),
        }
    }
}

impl Watcher for NetworkWatcher {
    fn pool(&mut self) {
        self.system.refresh_networks();

        for (iname, data) in self.system.networks() {
            if !self.inames.contains(iname) {
                continue;
            }
            if self.watch_rx {
                let key = (iname.to_string(), "received".to_string());
                let item = (SystemTime::now(), data.received());
                let values = self.metrics.entry(key).or_default();
                if values.len() == 50 {
                    values.pop_front();
                }
                values.push_back(item)
            }
            if self.watch_tx {
                let key = (iname.to_string(), "transmitted".to_string());
                let item = (SystemTime::now(), data.transmitted());
                let values = self.metrics.entry(key).or_default();
                if values.len() == 50 {
                    values.pop_front();
                }
                values.push_back(item)
            }
        }
    }

    fn get_metrics(&self) -> Vec<(&(String, String), f64)> {
        let mut get_metricsulations = vec![];
        for (k, v) in self.metrics.iter() {
            for op in self.transformations.iter() {
                get_metricsulations.push((k, op(v)));
            }
        }
        get_metricsulations
    }
}
