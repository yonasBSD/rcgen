use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
/// The error type of the rcgen crate
pub enum Error {
	/// The given certificate couldn't be parsed
	CouldNotParseCertificate,
	/// The given certificate signing request couldn't be parsed
	CouldNotParseCertificationRequest,
	/// The given key pair couldn't be parsed
	CouldNotParseKeyPair,
	#[cfg(feature = "x509-parser")]
	/// Invalid subject alternative name type
	InvalidNameType,
	/// Invalid ASN.1 string
	InvalidAsn1String(InvalidAsn1String),
	/// An IP address was provided as a byte array, but the byte array was an invalid length.
	InvalidIpAddressOctetLength(usize),
	/// There is no support for generating
	/// keys for the given algorithm
	KeyGenerationUnavailable,
	#[cfg(feature = "x509-parser")]
	/// Unsupported extension requested in CSR
	UnsupportedExtension,
	/// The requested signature algorithm is not supported
	UnsupportedSignatureAlgorithm,
	/// Unspecified `ring` error
	RingUnspecified,
	/// The `ring` library rejected the key upon loading
	RingKeyRejected(String),
	/// Time conversion related errors
	Time,
	#[cfg(feature = "pem")]
	/// Error from the pem crate
	PemError(String),
	/// Error generated by a remote key operation
	RemoteKeyError,
	/// Unsupported field when generating a CSR
	UnsupportedInCsr,
	/// Invalid certificate revocation list (CRL) next update.
	InvalidCrlNextUpdate,
	/// CRL issuer specifies Key Usages that don't include cRLSign.
	IssuerNotCrlSigner,
	#[cfg(not(feature = "crypto"))]
	/// Missing serial number
	MissingSerialNumber,
	/// X509 parsing error
	#[cfg(feature = "x509-parser")]
	X509(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::Error::*;
		match self {
			CouldNotParseCertificate => write!(f, "Could not parse certificate")?,
			CouldNotParseCertificationRequest => write!(
				f,
				"Could not parse certificate signing \
				request"
			)?,
			CouldNotParseKeyPair => write!(f, "Could not parse key pair")?,
			#[cfg(feature = "x509-parser")]
			InvalidNameType => write!(f, "Invalid subject alternative name type")?,
			InvalidAsn1String(e) => write!(f, "{e}")?,
			InvalidIpAddressOctetLength(actual) => {
				write!(f, "Invalid IP address octet length of {actual} bytes")?
			},
			KeyGenerationUnavailable => write!(
				f,
				"There is no support for generating \
				keys for the given algorithm"
			)?,
			UnsupportedSignatureAlgorithm => write!(
				f,
				"The requested signature algorithm \
				is not supported"
			)?,
			#[cfg(feature = "x509-parser")]
			UnsupportedExtension => write!(f, "Unsupported extension requested in CSR")?,
			RingUnspecified => write!(f, "Unspecified ring error")?,
			RingKeyRejected(e) => write!(f, "Key rejected by ring: {e}")?,

			Time => write!(f, "Time error")?,
			RemoteKeyError => write!(f, "Remote key error")?,
			#[cfg(feature = "pem")]
			PemError(e) => write!(f, "PEM error: {e}")?,
			UnsupportedInCsr => write!(f, "Certificate parameter unsupported in CSR")?,
			InvalidCrlNextUpdate => write!(f, "Invalid CRL next update parameter")?,
			IssuerNotCrlSigner => write!(
				f,
				"CRL issuer must specify no key usage, or key usage including cRLSign"
			)?,
			#[cfg(not(feature = "crypto"))]
			MissingSerialNumber => write!(f, "A serial number must be specified")?,
			#[cfg(feature = "x509-parser")]
			X509(e) => write!(f, "X.509 parsing error: {e}")?,
		};
		Ok(())
	}
}

impl std::error::Error for Error {}

/// Invalid ASN.1 string type
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidAsn1String {
	/// Invalid PrintableString type
	PrintableString(String),
	/// Invalid UniversalString type
	UniversalString(String),
	/// Invalid Ia5String type
	Ia5String(String),
	/// Invalid TeletexString type
	TeletexString(String),
	/// Invalid BmpString type
	BmpString(String),
}

impl fmt::Display for InvalidAsn1String {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use InvalidAsn1String::*;
		match self {
			PrintableString(s) => write!(f, "Invalid PrintableString: '{s}'")?,
			Ia5String(s) => write!(f, "Invalid IA5String: '{s}'")?,
			BmpString(s) => write!(f, "Invalid BMPString: '{s}'")?,
			UniversalString(s) => write!(f, "Invalid UniversalString: '{s}'")?,
			TeletexString(s) => write!(f, "Invalid TeletexString: '{s}'")?,
		};
		Ok(())
	}
}

/// A trait describing an error that can be converted into an `rcgen::Error`.
///
/// We use this trait to avoid leaking external error types into the public API
/// through a `From<x> for Error` implementation.
#[cfg(any(feature = "crypto", feature = "pem"))]
pub(crate) trait ExternalError<T>: Sized {
	fn _err(self) -> Result<T, Error>;
}
