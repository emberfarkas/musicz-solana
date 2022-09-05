use log::{info};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub(crate) struct Scan {
    // th: JoinHandle<Ou>
}

impl Scan {

    pub(crate) fn new() -> Self {
        Scan{}
    }

    pub (crate) fn start(&self) {
       self.run();
    }

    pub (crate) fn join(&self) {
        
    }

    fn run(&self) {
        tokio::spawn(async {
            info!("span");
        });
        info!("down");
    }
}