use std::io;
use std::thread;

pub fn create_thread<F>(name: String, f: F) -> io::Result<()>
where F: FnOnce(),
      F: Send + 'static,
{
    thread::Builder::new().name(name).spawn(f).map(|_| ())
}
