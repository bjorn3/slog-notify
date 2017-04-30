#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_notify;
extern crate slog_async;

use slog::Drain;

fn main(){
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = slog::Duplicate::new(drain, slog_notify::simple("Example simple")).fuse();
    
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_log = slog::Logger::root(drain, o!());
    
    crit!(root_log, "Hi");
}
