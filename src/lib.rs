#[macro_use]
extern crate slog;
extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::hints::NotificationHint;

struct NotifyDrain{
    name: &'static str
}

impl slog::Drain for NotifyDrain{
    type Error = String;
    
    fn log(&self, info: &slog::Record, _: &slog::OwnedKeyValueList) -> Result<(), Self::Error>{
        let summary = format!("{:?} {}@{}:{}", info.level(), info.file(), info.line(), info.column());
        let body = format!("{}", info.msg());
        
        let mut notification = Notification::new();
        notification
            .appname(self.name)
            .summary(&summary)
            .body(&body);
        
        if info.level().is_at_least(slog::Level::Warning){
            notification.hint(NotificationHint::Resident(true));
        }
        
        notification
            .show()
            .unwrap();
        Ok(())
    }
}

pub fn simple(name: &'static str) -> Box<slog::Drain<Error=String> + Send + Sync>{
    Box::new(NotifyDrain{
        name: name
    })
}