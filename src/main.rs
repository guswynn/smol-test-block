use std::future::Future;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::env;

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
    let count = if args.len() < 2 {
        1000
    } else {
        args[1].parse().unwrap()
    };

    smol::run(async {
        let t = Test { count };
        t.await
    });
}
