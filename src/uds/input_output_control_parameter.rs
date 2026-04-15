use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(
    uds,
    InputOutputControlParameter,
    InputOutputControlParameterByte
);
python_test!(
    uds,
    InputOutputControlParameter,
    ReturnControlToEcu,
    ShortTermAdjustment
);

/// `inputOutputControlParameter` data-parameter for
/// [`UdsCommand::InputOutputControlByIdentifier`](crate::uds::UdsCommand::InputOutputControlByIdentifier)
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputOutputControlParameter {
    /// Returns control of the input/output signals referenced by the dataIdentifier back to the ECU
    ReturnControlToEcu = 0x00,

    /// Resets the input/output signals referenced by the dataIdentifier to their default state
    ResetToDefault = 0x01,

    /// Freezes the current state of the input/output signals referenced by the dataIdentifier
    FreezeCurrentState = 0x02,

    /// Adjusts the input/output signals referenced by the dataIdentifier to the values
    /// provided in the `controlState` bytes of the `controlOptionRecord`
    ShortTermAdjustment = 0x03,
}
