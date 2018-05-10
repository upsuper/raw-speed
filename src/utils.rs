use std::io;
use std::thread;

pub fn create_thread(name: String, f: impl FnOnce() + Send + 'static) -> io::Result<()> {
    thread::Builder::new().name(name).spawn(f).map(|_| ())
}
