use std::{cell::Cell, rc::Rc};

use chrono::{DateTime, TimeDelta, Utc};

use crate::authorized::api::ControllerPoller;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum AuthLevel {
    #[default]
    View,
    Modify,
}

#[derive(Debug)]
pub struct AppState {
    pub input_username: String,
    pub api: ControllerPoller,
    pub input_password: String,
    pub enable_editing: Rc<Cell<bool>>,
    pub current_session: Option<SessionData>,
    pub sensor_data: SensorData,
}

#[derive(Debug)]
pub struct SensorData {
    pub is_on_battery: bool,
    pub is_alarm_enabled: ControlledValue<bool>,
}

pub enum ValueRef<'a, T> {
    Editable(&'a mut T),
    Readonly(&'a T),
}

#[derive(Default, Debug)]
pub struct ControlledValue<T> {
    enable_editing: Rc<Cell<bool>>,
    value: T,
}

#[derive(Debug)]
pub struct SessionData {
    pub username: String,
    pub auth_level: AuthLevel,
    pub begin_timestamp: DateTime<Utc>,
    pub timeout_time: TimeDelta,
}

impl<T: Default> ControlledValue<T> {
    pub fn from(edit_ref: &Rc<Cell<bool>>) -> Self {
        return Self {
            enable_editing: edit_ref.clone(),
            value: Default::default(),
        };
    }

    pub fn edit_value(&'_ mut self) -> ValueRef<'_, T> {
        if self.enable_editing.get() {
            ValueRef::Editable(&mut self.value)
        } else {
            ValueRef::Readonly(&self.value)
        }
    }
}
