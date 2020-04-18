pub mod async_thread;
pub use async_thread::{AsyncThread};

pub fn spawn<R>(func:fn()->R) -> impl Future
where R: 'static+Default+Send
{
    AsyncThreadthread::spawn()
}