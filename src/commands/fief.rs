use chrono::Duration;

pub fn add(_name: &str) {}

pub fn remove(_fief: &str) {}

pub fn check(_fief: &str) {}

pub fn close(_fief: &str, _dur: Option<Duration>) {}

pub fn open(_fief: &str) {}

pub fn set_time(_fief: &str, _dur: Duration) {}

pub fn set_name(_fief: &str, _new_name: &str) {}

pub fn info(_fief: &str) {}