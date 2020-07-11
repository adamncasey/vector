use super::controller::Controller;
use super::future::ResponseFuture;
use crate::sinks::util::retries2::RetryLogic;

use tower03::Service;

use futures::ready;
use std::{
    fmt,
    future::Future,
    mem,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::OwnedSemaphorePermit;

/// Enforces a limit on the concurrent number of requests the underlying
/// service can handle. Automatically expands and contracts the actual
/// concurrency limit depending on observed request response behavior.
#[derive(Debug)]
pub struct AutoConcurrencyLimit<S, L> {
    inner: S,
    controller: Arc<Controller<L>>,
    state: State,
}

enum State {
    Waiting(Pin<Box<dyn Future<Output = OwnedSemaphorePermit> + Send + 'static>>),
    Ready(OwnedSemaphorePermit),
    Empty,
}

impl<S, L> AutoConcurrencyLimit<S, L> {
    /// Create a new automated concurrency limiter.
    pub(crate) fn new(inner: S, logic: L, max: usize) -> Self {
        AutoConcurrencyLimit {
            inner,
            controller: Arc::new(Controller::new(max, logic, 1)),
            state: State::Empty,
        }
    }
}

impl<S, L, Request> Service<Request> for AutoConcurrencyLimit<S, L>
where
    S: Service<Request>,
    S::Error: Into<crate::Error>,
    L: RetryLogic<Response = S::Response>,
{
    type Response = S::Response;
    type Error = crate::Error;
    type Future = ResponseFuture<S::Future, L>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {
            self.state = match self.state {
                State::Ready(_) => return self.inner.poll_ready(cx).map_err(Into::into),
                State::Waiting(ref mut fut) => {
                    tokio::pin!(fut);
                    let permit = ready!(fut.poll(cx));
                    State::Ready(permit)
                }
                State::Empty => State::Waiting(Box::pin(self.controller.clone().acquire())),
            };
        }
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Make sure a permit has been acquired
        let permit = match mem::replace(&mut self.state, State::Empty) {
            // Take the permit.
            State::Ready(permit) => permit,
            // whoopsie!
            _ => panic!("max requests in-flight; poll_ready must be called first"),
        };

        self.controller.start_request();

        // Call the inner service
        let future = self.inner.call(request);

        ResponseFuture::new(future, permit, self.controller.clone())
    }
}

impl<S, L> Clone for AutoConcurrencyLimit<S, L>
where
    S: Clone,
    L: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            controller: self.controller.clone(),
            state: State::Empty,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Waiting(_) => f
                .debug_tuple("State::Waiting")
                .field(&format_args!("..."))
                .finish(),
            State::Ready(ref r) => f.debug_tuple("State::Ready").field(&r).finish(),
            State::Empty => f.debug_tuple("State::Empty").finish(),
        }
    }
}
