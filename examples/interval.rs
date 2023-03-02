use std::time::Duration;
use taq::*;
use tokio::time::sleep;

#[derive(Default)]
pub struct Interval {
    dur: Duration,
    cb: Option<Box<dyn FnMut() + Send>>,
}

impl Interval {
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
                _ = self.interval() => unreachable!(),
                withr = manager.poll() => withr?.with(&mut self).await,
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = taq::run(Interval::default());

    handle.run(job!(|interval| {
        interval.set_interval(|| println!("hello"), Duration::from_secs(1));
    }))?;

    futures::future::pending::<()>().await;

    unreachable!()
}
