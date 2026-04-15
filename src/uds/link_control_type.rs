use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, LinkControlType, LinkControlTypeByte);
python_test!(
    uds,
    LinkControlType,
    VerifyModeTransitionWithFixedParameter,
    TransitionMode
);

/// [`UdsCommand::LinkControl`](crate::uds::UdsCommand::LinkControl) sub-function definitions
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LinkControlType {
    /// Verifies whether a mode transition with a predefined parameter,
    /// specified by the `linkControlModeIdentifier` data-parameter, can be performed
    VerifyModeTransitionWithFixedParameter = 0x01,

    /// Verifies whether a mode transition with a specific parameter,
    /// specified by the `linkRecord` data-parameter, can be performed
    VerifyModeTransitionWithSpecificParameter = 0x02,

    /// Requests the server to transition the data link into the mode verified in the preceding request
    TransitionMode = 0x03,
}
