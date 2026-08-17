use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, ActivationType, ActivationTypeByte, display = @"14425352116563829080");
python_test!(doip, ActivationType, Default, WwhObd);

/// Represents routing types for activation requests.
///
/// Used to customize the routing type requested from the `DoIP` entity for different
/// scenarios.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "display", ignore_extra_doc_attributes)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActivationType {
    /// Default
    Default = 0x00,

    /// WWH-OBD
    ///
    /// ISO 13400-2:2025 broadens this entry to "diagnostic communication
    /// required by regulation (e.g. ISO 27145-1, ISO 20730-1)"; the wire
    /// value is unchanged.
    WwhObd = 0x01,

    /// Central Security
    ///
    /// ISO 13400-2 assigns central security `0xE0` (Table 23 in the
    /// 2012/2019 editions, Table 54 in the 2025 edition); `0x02..=0xDF` is
    /// ISO/SAE reserved. This was previously (incorrectly) `0x02`.
    CentralSecurity = 0xE0,
}
