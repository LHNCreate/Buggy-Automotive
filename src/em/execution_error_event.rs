use std::default::Default;
use super::deterministic_client::ExecutionError;
use std::str;

/// SWS_EM_02544
///
/// Represents an execution error event which happens in a Function Group
///

struct ExecutionErrorEvent {
    /// SWS_EM_02545
    ///
    /// wrap ExecutionError in Option
    execution_error: Option<ExecutionError>,

    /// SWS_EM_02546
    ///
    /// The function group in which the error occurred .
    function_group: String,

    /// SWS_EM_02560
    ///
    /// termination_handler - Callback which is called if ExecutionClient receives SIGTERM
    /// signal. The callback is executed in a background thread. A typical
    /// implementation of this callback will set a global flag (and potentially
    /// unblock other threads) to perform a graceful termination. The
    /// lifetime of the given function has to exceed the lifetime of the
    /// ExecutionClient object.
    ///
    termination_handler: Option<Box<dyn Fn()>>,
}

impl Default for ExecutionErrorEvent {
    fn default() -> Self {
        ExecutionErrorEvent {
            execution_error: None,
            function_group: "none".to_string(),
            termination_handler: None,
        }
    }
}

impl ExecutionErrorEvent {
    pub fn new() -> Self {
        ExecutionErrorEvent::default()
    }


    pub fn new_with_termination_handler<F>(handler: F) -> Self
    where
        F: Fn() + 'static,
    {
        ExecutionErrorEvent {
            termination_handler: Some(Box::new(handler)),
            ..Self::default()

        }
    }
}
