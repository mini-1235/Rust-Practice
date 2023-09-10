use futures::{
    future::{BoxFuture, FutureExt},
    // task::{waker_ref, ArcWake},
};
use scoped_tls::scoped_thread_local;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker,Wake},
};

use timer_future::Signal;
scoped_thread_local!(static SIGNAL:Arc<Signal>);
scoped_thread_local!(static RUNNABLE:Mutex<VecDeque<Arc<Task>>>);
unsafe impl Send for Task {}
unsafe impl Sync for Task {}
pub struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}
impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.signal.notify();
    }
}
pub struct TaskQueue {
    queue: RefCell<VecDeque<Rc<Task>>>,
}
impl TaskQueue {
    pub(crate) fn push(&self, runnable: Rc<Task>) {
        println!("add task");
        self.queue.borrow_mut().push_back(runnable);
    }

    pub(crate) fn pop(&self) -> Option<Rc<Task>> {
        println!("remove task");
        self.queue.borrow_mut().pop_front()
    }
}
pub struct Executor {
    task_queue: TaskQueue,
}
impl Executor {
    pub fn new() -> Self {
        Executor {
            task_queue: TaskQueue {
                queue: RefCell::new(VecDeque::new()),
            },
        }
    }
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let task = Rc::new(Task {
            future: RefCell::new(future.boxed()),
            signal: Arc::new(Signal::new()),
        });
        self.task_queue.push(task);
    }

    fn block_on<F: Future>(&self,mut fut: F) -> F::Output {
        let signal = Arc::new(Signal::new());
        let waker = Waker::from(Arc::clone(&signal));
        let mut context = Context::from_waker(&waker);
        let runnable: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::with_capacity(1024));
        let mut fut = unsafe { std::pin::Pin::new_unchecked(&mut fut) };
        SIGNAL.set(&signal, || {
            RUNNABLE.set(&runnable, || loop {
                if let Poll::Ready(output) = fut.as_mut().poll(&mut context) {
                    return output;
                }
                while let Some(task) = self.task_queue.pop() {
                    let waker = Waker::from(signal.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            })
        })
    }
}
async fn demo(tx: async_channel::Sender<()>) {
    println!("demo");
    let _ = tx.send(()).await;
}
fn main() {
    println!("multitasking");
    let executor = Executor::new();
    Executor::block_on(&executor,async {
        let (tx, rx) = async_channel::bounded(1);
        // Executor::spawn(demo(tx));
        Executor::spawn(&executor, demo(tx));
        println!("awaiting");
        let _ = rx.recv().await;
    });
}
