# slog-notify
Slog desktop notification bridge

```rust
#[macro_use]
extern crate slog;
extern crate slog_notify;

use slog::DrainExt;

fn main(){
    let drain = slog_notify::simple("Example simple");
    
    let root_log = slog::Logger::root(drain, o!());
    
    info!(root_log, "Information");
    crit!(root_log, "Critical message");
}

```
