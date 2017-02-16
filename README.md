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

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
