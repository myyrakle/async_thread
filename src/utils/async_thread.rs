use std::{
    future::{Future},
    task::{Context, Poll},
    pin::{Pin},
    default::{Default},
    sync::{Arc, Mutex},
    marker::{Send},
};

struct AsyncThreadState<R> where R: 'static+Default+Send
{
    return_value: R,
    done: bool,
}

impl<R> AsyncThreadState<R> where R: 'static+Default+Send
{
    fn new()->Self
    {
        AsyncThreadState{
            return_value: Default::default(), 
            done: false,
        }
    }
}

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

    fn poll(self: Pin<&mut Self>, _context: &mut Context) -> Poll<Self::Output>
    {
        let locked = self.state.lock().expect("붐");
        match locked.done
        {
            true => Poll::Ready(locked.return_value), // ?? How to solve
            false => Poll::Pending,
        }
    }
}


