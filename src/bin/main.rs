use sysmetrics::{
    builder::{networkwatcherbuilder, Builder},
    operations,
    watcher::Watcher,
};
use ubyte::ByteUnit;

fn main() {

    let mut network_watcher = networkwatcherbuilder::NetworkWatcherBuilder::new()
        .inames(vec!["en0".to_string()])
        .watch_rx(true)
        .watch_tx(true)
        .unit(ByteUnit::MiB)
        .transformations(vec![Box::new(operations::mean)])
        .build();
    loop {
        network_watcher.pool();
        network_watcher.get_metrics();
        sleep(Duration::from_millis(5));
    }
}
