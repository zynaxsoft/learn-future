use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

struct Kos {
    shared: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Kos {
    fn new() -> Self {
        let shared = SharedState {
            completed: false,
            waker: None,
        };
        Self {
            shared: Arc::new(Mutex::new(shared)),
        }
    }
}

impl Future for Kos {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Start");
        let mut shared = self.shared.lock().unwrap();
        if shared.completed {
            Poll::Ready("Yes".to_owned())
        } else {
            println!("you are stucking here with me");
            shared.waker = Some(cx.waker().clone());
            println!("yep pending");
            Poll::Pending
        }
    }
}

fn get_that_string() -> Kos {
    Kos::new()
}

#[tokio::main]
async fn main() {
    let k = get_that_string();
    let shared = k.shared.clone();
    thread::spawn(move || {
        thread::sleep(Duration::new(2, 0));
        let mut shared = shared.lock().unwrap();
        // shared.completed = true;
        if let Some(waker) = shared.waker.take() {
            println!("hey wake up");
            waker.wake();
        }
    });
    println!("{}", k.await);
}
