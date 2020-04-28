use std::future::Future;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::env;

use tokio::runtime::Runtime;


struct Test {
    count: i64,
}

impl Future for Test {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        self.count -= 1;

        println!("Current countdown: {}", self.count);

        if self.count > 0 {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: i64 = if args.len() < 2 {
        1000
    } else {
        args[1].parse().unwrap()
    };

    let mut rt = Runtime::new().unwrap();

    rt.block_on(async move {
        let t = Test { count };
        t.await
    });
}
