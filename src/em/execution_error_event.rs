use std::default::Default;
use super::execution_client::ExecutionState;



/// SWS_EM_02541
///
/// Represents the execution error
///
pub(crate) type ExecutionError = u32;




/// SWS_EM_02544
///
/// Represents an execution error event which happens in a Function Group
///

pub struct ExecutionErrorEvent {
    /// SWS_EM_02545
    ///
    /// wrap ExecutionError in Option
    execution_error: Option<ExecutionError>,

    /// SWS_EM_02546
    ///
    /// The function group in which the error occurred .
    function_group: String,

}

impl Default for ExecutionErrorEvent {
    fn default() -> Self {
        ExecutionErrorEvent {
            execution_error: None,
            function_group: "none".to_string(),
        }
    }
}

impl ExecutionErrorEvent {
    pub fn new() -> Self {
        ExecutionErrorEvent::default()
    }

}
