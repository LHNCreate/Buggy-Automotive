use super::deterministic_client::ExecutionError;
use std::str;


/// SWS_EM_02544
///
/// Represents an execution error event which happens in a Function Group
struct ExectuionErrorEvent{
    /// SWS_EM_02545
    ///
    /// wrap ExecutionError in Option
    execution_error:Option<ExecutionError>,
}

impl ExectuionErrorEvent {
    pub fn new() -> Self{
        ExectuionErrorEvent{
            execution_error:None
        }
    }
}
