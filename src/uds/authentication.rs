use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(
    uds,
    AuthenticationSubFunction,
    AuthenticationSubFunctionByte
);
python_test!(
    uds,
    AuthenticationSubFunction,
    DeAuthenticate,
    VerifyCertificateUnidirectional
);

/// UDS Diagnostic Authentication subfunctions. Handled by SID 0x29
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AuthenticationSubFunction {
    /// Terminates the authentication session
    DeAuthenticate = 0x00,

    /// Initiates unidirectional certificate verification where the server validates the client
    VerifyCertificateUnidirectional = 0x01,

    /// Initiates mutual, bidirectional authentication between a diagnostic tester and an ECU
    VerifyCertificateBidirectional = 0x02,

    /// Proves that the client possesses the private key corresponding to the certificate
    ProofOfOwnership = 0x03,

    /// Initiates configuration mode
    AuthenticationConfiguration = 0x08,
}
