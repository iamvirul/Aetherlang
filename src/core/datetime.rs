use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use std::fmt;

#[derive(Clone, Debug)]
pub struct AetherDateTime {
    inner: DateTime<Utc>,
}

impl AetherDateTime {
    pub fn now() -> Self {
        Self {
            inner: Utc::now(),
        }
    }

    pub fn from_timestamp(timestamp: i64) -> Option<Self> {
        DateTime::from_timestamp(timestamp, 0).map(|dt| Self { inner: dt })
    }

    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    pub fn month(&self) -> u32 {
        self.inner.month()
    }

    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    pub fn hour(&self) -> u32 {
        self.inner.hour()
    }

    pub fn minute(&self) -> u32 {
        self.inner.minute()
    }

    pub fn second(&self) -> u32 {
        self.inner.second()
    }

    pub fn to_local(&self) -> DateTime<Local> {
        self.inner.with_timezone(&Local)
    }

    pub fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    pub fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }
}

impl fmt::Display for AetherDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner.format("%Y-%m-%d %H:%M:%S UTC"))
    }
} 