#[repr(u32)]
pub enum ExecErrc {
    KcommunicationError = 3,
    KmetaModelError = 4,
    Kcancelled = 5,
    Kfailed = 6,
    KfailedUnexpected,
    KfailedUnexpectedTerminationOnEnter = 8,
    KinvalidTransition = 9,
    KnoTimeStamp = 12,
    KcycleOverrun = 13,
    KintegrityOrAuthenticityCheckFailed = 14,
    KfailedUnexpectedTermination = 15,
    KinvalidArgument = 16
}