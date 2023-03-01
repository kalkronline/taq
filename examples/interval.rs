use std::{hint::unreachable_unchecked, time::Duration};
use taq::{Task, TaskManager};
use tokio::time::sleep;

pub struct Interval {
    dur: Duration,
    cb: Option<Box<dyn FnMut() + Send>>,
}

impl Interval {
    pub fn new() -> Self {
        let dur = Duration::ZERO; // doesn't really matter
        let cb = None;
        Self { dur, cb }
    }

    pub fn set_interval<F>(&mut self, cb: F, dur: Duration)
    where
        F: FnMut() + Send + 'static,
    {
        self.dur = dur;
        self.cb = Some(Box::new(cb));
    }

    async fn interval(&mut self) -> ! {
        // will loop until future is cancelled
        if let Some(cb) = &mut self.cb {
            loop {
                sleep(self.dur).await;
                cb();
            }
        } else {
            futures::future::pending::<()>().await;
            unreachable!();
        }
    }
}

#[async_trait::async_trait]
impl Task for Interval {
    async fn task(mut self, mut manager: TaskManager<Self>) -> Option<()> {
        loop {
            tokio::select! {
                biased;
                _ = self.interval() => unreachable!(),
                giver = manager.self_req() => giver?.give(&mut self).await,
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let handle = taq::spawn(Interval::new());

    handle
        .run(taq::futgen!(|t| {
            t.set_interval(|| println!("Hello, world!"), Duration::from_secs(1));
        }))
        .unwrap()
        .await
        .unwrap();
}
