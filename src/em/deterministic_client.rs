

/// SWS_EM_02201
///
/// Defines the return codes for WaitForActivation operations. Scoped Enumeration of uint32_t .
///
///
#[deprecated(since = "Adaptive Autosar R23-11")]
#[repr(u32)]
enum ActivationReturnType {
    KregisterServices, //application shall register communication services
    KServiceDiscovery, //application shall do communication service discovery
    Kinit,             //application shall initialize its internal data structures
    Krun,              //application shall perform its normal operation
    Kterminate,        //deterministic execution shall terminate
}

/// SWS_EM_02541
///
/// Represents the execution error
///
pub(crate) type ExecutionError = u32;


