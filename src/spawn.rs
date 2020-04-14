pub mod async_thread;
pub use async_thread::{AsyncThread};

pub fn spawn(func:fn()->R) where R: 'static+Default+Send -> impl Future
{
    AsyncThreadthread::spawn()
}