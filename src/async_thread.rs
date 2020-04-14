use std::{
    future::{Future},
    task::{Context, Poll},
    pin::{Pin},
    default::{Default},
    sync::{Arc, Mutex},
    marker::{Send},
};

mod async_state;
use async_state::*;

pub struct AsyncThread<R> where R: 'static+Default+Send
{
    state: Arc<Mutex< AsyncThreadState<R> >>,
}


impl<R> AsyncThread<R> where R: 'static+Default+Send {
    fn spawn(func:fn()->R) -> impl Future {
        let mutex_state = Arc::new(Mutex::new(
            AsyncThreadState::<R>::new()
        ));

        let borrow = mutex_state.clone();

        std::thread::spawn(move || {
           borrow.lock().expect("실패").return_value = func();
           borrow.lock().expect("실패").done = true;
        });

        AsyncThread { state: mutex_state }
    }
}

impl<R> Future for AsyncThread<R> where R: 'static+Default+Send
{
    type Output = R;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output>
    {
        match self.as_mut().lock().expect("붐") {
            true => Poll::Ready(self.return_value.into_inner().unwrap()), 
            false => Poll::Pending,
        }
    }
}
