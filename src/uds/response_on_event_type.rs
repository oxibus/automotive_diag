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
/// These variants encode bits 5:0 of the sub-function byte only.
/// Bit 6 is an orthogonal `storeEvent` flag handled separately by [`ResponseOnEventSubFunction`].
/// Bit 7 is the standard `suppressPosRspMsgIndicationBit` handled at the protocol layer.
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

    /// Configures the server to send a response whenever a DTC status change matches the provided `DTCStatusMask`
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

    /// Requests the server to report DTC record information when a DTC status change matches the provided `DTCStatusMask`
    ReportDtcRecordInformationOnDtcStatusChange = 0x09,
}

/// Decoded sub-function byte for [`UdsCommand::ResponseOnEvent`](crate::uds::UdsCommand::ResponseOnEvent).
///
/// The raw sub-function byte packs two orthogonal fields:
/// - Bits 5:0 — the event type, decoded into [`ResponseOnEventTypeByte`]
/// - Bit 6 — the `storeEvent` flag: if `true`, the server persists the event setup across power cycles
///   (only permitted for [`ResponseOnEventType::StartResponseOnEvent`] and
///   [`ResponseOnEventType::StopResponseOnEvent`] in the default session with an infinite event window)
///
/// Use [`From<u8>`] to decode a raw byte and [`From<ResponseOnEventSubFunction> for u8`] to encode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseOnEventSubFunction {
    /// The event type decoded from bits 5:0 of the sub-function byte.
    pub event_type: ResponseOnEventTypeByte,
    /// When `true` (bit 6 = 1), the server stores the event setup and resumes it after a power cycle.
    pub store_event: bool,
}

impl From<u8> for ResponseOnEventSubFunction {
    fn from(byte: u8) -> Self {
        Self {
            event_type: ResponseOnEventTypeByte::from(byte & 0x3F),
            store_event: (byte & 0x40) != 0,
        }
    }
}

impl From<ResponseOnEventSubFunction> for u8 {
    fn from(v: ResponseOnEventSubFunction) -> Self {
        let base: u8 = v.event_type.into();
        base | if v.store_event { 0x40 } else { 0x00 }
    }
}

#[cfg(test)]
mod response_on_event_sub_function_tests {
    use super::*;
    use crate::ByteWrapper;

    #[test]
    fn roundtrip() {
        // Bit 7 is the suppressPosRspMsgIndicationBit, handled at the protocol layer before
        // this struct is used. Only bytes 0x00..=0x7F are within this struct's scope.
        for byte in 0x00u8..=0x7F {
            let decoded = ResponseOnEventSubFunction::from(byte);
            let encoded = u8::from(decoded);
            assert_eq!(byte, encoded, "{byte:#04X} → {decoded:?} → {encoded:#04X}");
        }
    }

    #[test]
    fn store_event_bit_is_isolated() {
        let with_store = ResponseOnEventSubFunction::from(0x41);
        let without_store = ResponseOnEventSubFunction::from(0x01);
        assert!(with_store.store_event);
        assert!(!without_store.store_event);
        assert_eq!(
            with_store.event_type,
            ByteWrapper::Standard(ResponseOnEventType::OnDtcStatusChange)
        );
        assert_eq!(with_store.event_type, without_store.event_type);
    }
}
