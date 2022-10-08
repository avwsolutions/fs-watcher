use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter, warn};


fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "fs-watcher".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Warn))
        .expect("could not register logger");

    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            // Ok(event) => println!("changed: {:?}", event),
            Ok(event) => logevent(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn logevent(event:notify::Event) { 
    warn!("change occured: {:?}", event);
}
