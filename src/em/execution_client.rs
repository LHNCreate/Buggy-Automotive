use crate::em::exec_error_domain::ExecErrc;

/// SWS_EM_02000
///
/// Defines the internal states of a Process
///
///
#[repr(u8)]
pub enum ExecutionState{
    Krunning
}

pub struct ExecutionClient{
    execution_state: Option<ExecutionState>,
    /// SWS_EM_02560
    ///
    /// termination_handler - Callback which is called if ExecutionClient receives SIGTERM
    /// signal. The callback is executed in a background thread. A typical
    /// implementation of this callback will set a global flag (and potentially
    /// unblock other threads) to perform a graceful termination. The
    /// lifetime of the given function has to exceed the lifetime of the
    /// ExecutionClient object.
    ///
    pub termination_handler: Option<Box<dyn Fn()>>,
}

impl Default for ExecutionClient {
    fn default() -> Self {
        ExecutionClient {
            execution_state: None,
            termination_handler: None,
        }
    }


}

impl ExecutionClient {
    pub fn new() -> Self {
        ExecutionClient::default()
    }
    pub fn termination<F>(handler: F) -> Self
    where
        F: Fn() + 'static,
    {
        ExecutionClient {
            termination_handler: Some(Box::new(handler)),
            ..Self::default()

        }
    }



    //todo SWS_EM_02003

    // pub fn report_execution_state(&mut self,state: ExecutionState) -> Result<(),ExecErrc>{
    //
    // }


}