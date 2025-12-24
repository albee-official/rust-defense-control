use crate::authorized::api::Api;
use crate::authorized::serial_connection::PollResult;
use battery::Manager;
use chrono::{DateTime, TimeDelta, Utc};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum AuthLevel {
    #[default]
    View,
    Modify,
}

#[derive(Debug)]
pub struct AppState {
    pub input_username: String,
    pub input_password: String,
    pub api: Api,
    pub last_poll_result: PollResult,
    pub battery_manager: Manager,
    pub current_session: Option<SessionData>,
    pub alarm_end_time: Option<DateTime<Utc>>,
}

pub enum ValueRef<'a, T> {
    Editable(&'a mut T),
    Readonly(&'a T),
}

#[derive(Debug)]
pub struct SessionData {
    pub username: String,
    pub auth_level: AuthLevel,
    pub begin_timestamp: DateTime<Utc>,
    pub timeout_time: TimeDelta,
}
