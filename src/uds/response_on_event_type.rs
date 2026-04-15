use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, ResponseOnEventType, ResponseOnEventTypeByte);
python_test!(
    uds,
    ResponseOnEventType,
    StopResponseOnEvent,
    StartResponseOnEvent
);

/// [`UdsCommand::ResponseOnEvent`](crate::uds::UdsCommand::ResponseOnEvent) sub-function definitions
///
/// Note: bit 6 of the sub-function byte is an orthogonal `storeEvent` flag (0 = do not store,
/// 1 = store and resume after power-up) and is not encoded in these variants.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResponseOnEventType {
    /// Stops the server from sending responses on event; the configured event logic is preserved
    /// and can be restarted with `StartResponseOnEvent`
    StopResponseOnEvent = 0x00,

    /// Configures the server to send a response whenever a DTC status change matches the provided DTCStatusMask
    OnDtcStatusChange = 0x01,

    /// Configures the server to send a response when an internal data record identified by a dataIdentifier changes
    OnChangeOfDataIdentifier = 0x03,

    /// Requests the server to report all events currently activated via this service
    ReportActivatedEvents = 0x04,

    /// Activates the previously configured event logic and starts the event window timer
    StartResponseOnEvent = 0x05,

    /// Clears the configured event logic and stops the server from sending responses on event
    ClearResponseOnEvent = 0x06,

    /// Configures the server to send a response when a specific measurement value comparison result is positive
    OnComparisonOfValues = 0x07,

    /// Requests the server to report the most recent DTC with a testFailed or confirmedDTC bit transition from 0 to 1
    ReportMostRecentDtcOnStatusChange = 0x08,

    /// Requests the server to report DTC record information when a DTC status change matches the provided DTCStatusMask
    ReportDtcRecordInformationOnDtcStatusChange = 0x09,
}
