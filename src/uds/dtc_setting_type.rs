use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, DtcSettingType, DtcSettingTypeByte);
python_test!(uds, DtcSettingType, On, Off);

/// [`UdsCommand::ControlDTCSetting`](crate::uds::UdsCommand::ControlDTCSetting) sub-function definitions
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DtcSettingType {
    /// Resumes the updating of DTC status bits according to normal operating conditions
    On = 0x01,

    /// Stops the updating of DTC status bits in the server
    Off = 0x02,
}
