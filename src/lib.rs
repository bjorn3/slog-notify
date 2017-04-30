/*Copyright 2017 bjorn3

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.*/

extern crate slog;
extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::hints::NotificationHint;

pub struct Notify{
    name: &'static str
}

impl slog::Drain for Notify{
    type Err = String;
    type Ok = ();
    
    fn log(&self, info: &slog::Record, _: &slog::OwnedKVList) -> Result<Self::Ok, Self::Err>{
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

pub fn simple(name: &'static str) -> Notify {
    Notify{
        name: name
    }
}
