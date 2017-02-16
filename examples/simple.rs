#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_notify;

use slog::DrainExt;

fn main(){
    let drain = slog::Duplicate::new(slog_term::streamer().compact().build(), slog_notify::simple("Example simple")).fuse();
    
    let root_log = slog::Logger::root(drain, o!());
    
    crit!(root_log, "Hi");
}
