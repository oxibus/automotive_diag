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

/// [`UdsCommand::Authentication`](crate::uds::UdsCommand::Authentication) sub-function definitions
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

    /// Transfers a certificate to the server for processing without a challenge-response sequence,
    /// enabling additional rights activation or signed-data proof verification
    TransmitCertificate = 0x04,

    /// Initiates the challenge-response (ACR) authentication flow by requesting a server-generated challenge
    RequestChallengeForAuthentication = 0x05,

    /// Submits the client-side proof of ownership to the server for unidirectional verification
    /// in an ACR authentication flow
    VerifyProofOfOwnershipUnidirectional = 0x06,

    /// Submits the client-side proof of ownership to the server and requests the corresponding
    /// server-side proof for bidirectional ACR authentication
    VerifyProofOfOwnershipBidirectional = 0x07,

    /// Requests the server to indicate its supported authentication configuration (APCE or ACR)
    AuthenticationConfiguration = 0x08,
}
