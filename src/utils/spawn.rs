mod async_thread;
use async_thread::AsyncThread;

// simple shortcut function
pub fn spawn<R>(func:fn()->R) -> impl Future
where R: 'static+Default+Send
{
    AsyncThread::spawn(func)
}