use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, PeriodicTransmissionMode, PeriodicTransmissionModeByte);
python_test!(uds, PeriodicTransmissionMode, SendAtSlowRate, StopSending);

/// `transmissionMode` data-parameter for
/// [`UdsCommand::ReadDataByPeriodicIdentifier`](crate::uds::UdsCommand::ReadDataByPeriodicIdentifier)
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PeriodicTransmissionMode {
    /// Requests periodic transmission at a slow, vehicle manufacturer-defined rate
    SendAtSlowRate = 0x01,

    /// Requests periodic transmission at a medium, vehicle manufacturer-defined rate
    SendAtMediumRate = 0x02,

    /// Requests periodic transmission at a fast, vehicle manufacturer-defined rate
    SendAtFastRate = 0x03,

    /// Stops the server from periodically transmitting the specified data record(s)
    StopSending = 0x04,
}
