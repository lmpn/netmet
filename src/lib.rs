pub mod builder;
pub mod operations;
pub mod types;
pub mod watcher;

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use ubyte::ByteUnit;

    use crate::{
        builder::{networkwatcherbuilder, Builder},
        operations,
        watcher::Watcher,
    };

    #[test]
    fn exploration() {
        let mut network_watcher = networkwatcherbuilder::NetworkWatcherBuilder::new()
            .inames(vec!["en0".to_string()])
            .watch_rx(true)
            .watch_tx(true)
            .unit(ByteUnit::MiB)
            .transformations(vec![Box::new(|metrics| operations::mean(metrics))])
            .build();
        let results = vec![
            ("en0".to_string(), "transmitted".to_string()),
            ("en0".to_string(), "received".to_string()),
        ];
        for _ in 0..50 {
            network_watcher.pool();
        }

        for _ in 0..1000 {
            network_watcher.pool();
            sleep(Duration::from_millis(5));
        }
        let v = network_watcher.get_metrics();
        v.iter().for_each(|((a, b), value)| {
            let iter = results.iter().find(|(iface, name)| iface == a && name == b);
            println!("{:?} {:?} {:?}", a, b, value);
            assert_ne!(iter, None);
            assert!(*value > 0.0);
        });
    }
}
