use taq::TaskManager;

struct Logger {
    count: usize,
}

impl Logger {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn log(&mut self, msg: &str) {
        self.count += 1;
        println!("LOG {}: {}", self.count, msg);
    }
}

#[async_trait::async_trait]
impl taq::Task for Logger {
    async fn task(mut self, mut mgr: TaskManager<Self>) -> Option<()> {
        while let Some(run) = mgr.poll().await {
            run.with(&mut self).await;
        }
        Some(())
    }
}

#[tokio::main]
async fn main() {
    use taq::HandleExt;

    taq::run_global(Logger::new());

    let a = tokio::spawn(async move {
        let handle = taq::global::get_handle::<Logger>().unwrap();

        handle
            .recv(taq::job!(|logger| {
                logger.log("A");
            }))
            .unwrap()
            .await
            .unwrap();
    });

    let b = tokio::spawn(async move {
        let handle = taq::global::get_handle::<Logger>().unwrap();

        handle
            .recv(taq::job!(|logger| {
                logger.log("B");
            }))
            .unwrap()
            .await
            .unwrap();
    });

    let _ = tokio::join!(a, b);
}
