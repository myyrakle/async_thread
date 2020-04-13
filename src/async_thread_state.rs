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