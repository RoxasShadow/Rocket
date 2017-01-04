use std::ops::Deref;

use request::{self, FromRequest, Request};
use outcome::Outcome;
use http::Status;
use state;

pub struct State<T: Send + Sync + 'static>(&'static T);

impl<T: Send + Sync + 'static> State<T> {
    pub fn inner(&self) -> &'static T {
        self.0
    }
}

impl<'a, 'r, T: Send + Sync + 'static> FromRequest<'a, 'r> for State<T> {
    type Error = ();

    fn from_request(_: &'a Request<'r>) -> request::Outcome<State<T>, ()> {
        match state::try_get::<T>() {
            Some(state) => Outcome::Success(State(state)),
            None => {
                error_!("Attempted to retrieve unmanaged state!");
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}

impl<T: Send + Sync + 'static> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0
    }
}
