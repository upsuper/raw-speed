use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::iter;
use std::net::TcpStream;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(not(target_env = "musl"))]
use console::Term;
use crossbeam;
use humansize::FileSize;
use humansize::file_size_opts::BINARY;

use protocol::{self, Mode};

#[derive(Clone, Copy)]
struct Sample {
    time: Instant,
    bytes: usize,
}

impl Sample {
    fn now_from(bytes: &AtomicUsize) -> Self {
        Sample {
            time: Instant::now(),
            bytes: bytes.swap(0, Ordering::Relaxed),
        }
    }
}

/// Given the start time and the samples, computes the average bytes
/// per second.
fn compute_average<'a, I: 'a>(since: Instant, samples: I) -> usize
where I: Iterator<Item=&'a Sample>
{
    let (bytes, last_time) = samples.fold(
        (0, since),
        |(b, _), s| (b + s.bytes, s.time)
    );
    if last_time == since {
        return 0;
    }
    let duration = last_time - since;
    let seconds = duration.as_secs() as f32 +
        (duration.subsec_nanos() as f32 * 1e-9f32);
    (bytes as f32 / seconds).round() as usize
}

/// Pops samples from the given deque until the first sample is only
/// one which is not taken after the given time. (Thus the first sample
/// can be used as the reference start point.
fn pop_samples_before(samples: &mut VecDeque<Sample>, until: Instant) {
    while samples.len() >= 2 {
        if samples[1].time > until {
            break;
        }
        samples.pop_front();
    }
}

pub fn run(addr: &str, port: u16, mode: Mode) {
    assert!(!mode.is_empty());

    let mut conn = TcpStream::connect((addr, port))
        .expect("Failed to connect to server");
    conn.write_all(protocol::MAGIC_NUMBER)
        .expect("Failed to send magic number");
    conn.write_all(&[mode.bits()])
        .expect("Failed to send mode");

    conn.flush().expect("Failed to flush data");
    let start_time = Instant::now();
    #[cfg(not(target_env = "musl"))]
    let term = Term::stdout();

    let up_bytes = AtomicUsize::new(0);
    let down_bytes = AtomicUsize::new(0);
    crossbeam::scope(|scope| {
        let mut conn = Some(conn);
        if mode.contains(Mode::UP) {
            let conn = if mode.contains(Mode::DOWN) {
                conn.as_ref().unwrap().try_clone()
                    .expect("Failed to clone conection for upstream")
            } else {
                conn.take().unwrap()
            };
            scope.spawn(|| handle_upstream(conn, &up_bytes));
        }
        if mode.contains(Mode::DOWN) {
            let conn = conn.take().unwrap();
            scope.spawn(|| handle_downstream(conn, &down_bytes));
        }

        let sample_interval = Duration::new(1, 0);
        let mut up_sample = Sample { time: start_time, bytes: 0 };
        let mut up_samples_5s = VecDeque::with_capacity(6);
        let mut up_samples_1m = VecDeque::with_capacity(61);
        let mut down_sample = Sample { time: start_time, bytes: 0 };
        let mut down_samples_5s = VecDeque::with_capacity(6);
        let mut down_samples_1m = VecDeque::with_capacity(61);
        up_samples_5s.push_back(up_sample);
        up_samples_1m.push_back(up_sample);
        down_samples_5s.push_back(down_sample);
        down_samples_1m.push_back(down_sample);
        thread::sleep(start_time + sample_interval - Instant::now());
        loop {
            let new_up_sample = Sample::now_from(&up_bytes);
            let new_down_sample = Sample::now_from(&down_bytes);

            fn compute_avg(
                samples: &mut VecDeque<Sample>,
                new: &Sample,
                duration: Duration
            ) -> usize {
                samples.push_back(*new);
                pop_samples_before(samples, new.time - duration);
                compute_average(samples[0].time, samples.iter().skip(1))
            }
            fn print_avgs(
                dir: &str,
                sample: &mut Sample,
                new_sample: &Sample,
                samples_5s: &mut VecDeque<Sample>,
                samples_1m: &mut VecDeque<Sample>,
            ) {
                let avg_1s = compute_average(sample.time, iter::once(new_sample));
                let avg_5s = compute_avg(samples_5s, new_sample,
                                        Duration::new(5, 0));
                let avg_1m = compute_avg(samples_1m, new_sample,
                                        Duration::new(60, 0));
                *sample = *new_sample;
                println!(
                    "{}: 1s avg: {}/s, 5s avg: {}/s, 1min avg: {}/s",
                    dir,
                    avg_1s.file_size(BINARY).unwrap(),
                    avg_5s.file_size(BINARY).unwrap(),
                    avg_1m.file_size(BINARY).unwrap(),
                );
            }

            let mut _lines = 0;
            if mode.contains(Mode::UP) {
                print_avgs("Up", &mut up_sample, &new_up_sample,
                           &mut up_samples_5s, &mut up_samples_1m);
                _lines += 1;
            }
            if mode.contains(Mode::DOWN) {
                print_avgs("Down", &mut down_sample, &new_down_sample,
                           &mut down_samples_5s, &mut down_samples_1m);
                _lines += 1;
            }

            thread::sleep(sample_interval);
            #[cfg(not(target_env = "musl"))]
            term.clear_last_lines(_lines).unwrap();
        }
    });
}

fn handle_upstream(mut socket: TcpStream, counter: &AtomicUsize) {
    impl_send!(socket: n => { counter.fetch_add(n, Ordering::Relaxed); });
}

fn handle_downstream(mut socket: TcpStream, counter: &AtomicUsize) {
    impl_recv!(socket: n => { counter.fetch_add(n, Ordering::Relaxed); });
}
