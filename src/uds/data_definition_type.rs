use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, DataDefinitionType, DataDefinitionTypeByte);
python_test!(
    uds,
    DataDefinitionType,
    DefineByIdentifier,
    ClearDynamicallyDefinedDataIdentifier
);

/// [`UdsCommand::DynamicallyDefineDataIdentifier`](crate::uds::UdsCommand::DynamicallyDefineDataIdentifier) sub-function definitions
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataDefinitionType {
    /// Defines the dynamic data identifier via a reference to an existing data identifier
    DefineByIdentifier = 0x01,

    /// Defines the dynamic data identifier via an absolute memory address and length
    DefineByMemoryAddress = 0x02,

    /// Clears the specified dynamically defined data identifier; the server responds positively
    /// even if the identifier does not currently exist
    ClearDynamicallyDefinedDataIdentifier = 0x03,
}
