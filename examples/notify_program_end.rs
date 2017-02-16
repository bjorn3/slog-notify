#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_notify;

use std::process::Command;

use slog::DrainExt;

fn main(){
    let drain = slog::Duplicate::new(slog_term::streamer().compact().build(), slog_notify::simple("Notify program end")).fuse();
    
    let root_log = slog::Logger::root(drain, o!());
    
    let cmd = std::env::args().skip(1).collect::<Vec<_>>();
    let cmd_str = cmd.iter().fold(String::new(), |acc, x|acc+" "+x);
    
    let ecode = Command::new(cmd[0].clone())
        .args(&cmd[1..])
        .spawn()
        .expect("Failed to spawn")
        .wait()
        .expect("Failed to wait on for process")
        .code()
        .expect("Failed to get error code");
    
    if ecode == 0{
        info!(root_log, "Process \"{}\" ended", cmd_str);
    }else{
        error!(root_log, "Process \"{}\" errored with code {}", cmd_str, ecode);
    }
}
