use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, FileTransferMode, FileTransferModeByte);
python_test!(uds, FileTransferMode, AddFile, ReadFile);

/// `modeOfOperation` data-parameter for
/// [`UdsCommand::RequestFileTransfer`](crate::uds::UdsCommand::RequestFileTransfer)
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitEncode))]
#[cfg_attr(feature = "bin-proto", derive(bin_proto::BitDecode))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileTransferMode {
    /// Downloads (adds) a new file to the server at the path specified by `filePathAndName`
    AddFile = 0x01,

    /// Deletes the file at the path specified by `filePathAndName` from the server
    DeleteFile = 0x02,

    /// Downloads and replaces the file at the path specified by `filePathAndName`;
    /// adds the file if it does not already exist on the server
    ReplaceFile = 0x03,

    /// Uploads (reads) the file at the path specified by `filePathAndName` from the server
    ReadFile = 0x04,

    /// Reads the directory listing at the path specified by `filePathAndName` from the server;
    /// `filePathAndName` indicates a directory, not a file name
    ReadDir = 0x05,

    /// Resumes a previously interrupted file download at the file position returned by the server
    ResumeFile = 0x06,
}
