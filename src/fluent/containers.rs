use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

pub trait Innable<T> {
    fn in_arc(self) -> Arc<T>;
    fn in_rc(self) -> Rc<T>;
    fn in_mutex(self) -> Mutex<T>;
}

impl<T> Innable<T> for T {
    fn in_arc(self) -> Arc<T> {
        Arc::new(self)
    }

    fn in_rc(self) -> Rc<T> {
        Rc::new(self)
    }

    fn in_mutex(self) -> Mutex<T> {
        Mutex::new(self)
    }
}
