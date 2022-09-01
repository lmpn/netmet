use ubyte::ByteUnit;

use crate::{types::networkwatcher::NetworkWatcher, types::Transformation};

#[derive(Default)]
pub struct NetworkWatcherBuilder {
    network_watcher: NetworkWatcher,
}
impl NetworkWatcherBuilder {
    pub fn new() -> Self {
        NetworkWatcherBuilder::default()
    }

    pub fn inames(mut self, arg: Vec<String>) -> Self {
        self.network_watcher.inames(arg);
        self
    }

    pub fn watch_rx(mut self, arg: bool) -> Self {
        self.network_watcher.watch_rx(arg);
        self
    }

    pub fn watch_tx(mut self, arg: bool) -> Self {
        self.network_watcher.watch_tx(arg);
        self
    }

    pub fn unit(mut self, arg: ByteUnit) -> Self {
        self.network_watcher.unit(arg);
        self
    }
    pub fn transformations(mut self, arg: Vec<Transformation>) -> Self {
        self.network_watcher.transformations(arg);
        self
    }
}

impl super::Builder<NetworkWatcher> for NetworkWatcherBuilder {
    fn build(self) -> NetworkWatcher {
        self.network_watcher
    }
}
