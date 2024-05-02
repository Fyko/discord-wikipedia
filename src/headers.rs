use headers::{Header, HeaderName, HeaderValue};

/// The `x-signature-timestamp` header.
pub static X_SIGNATURE_TIMESTAMP: HeaderName = HeaderName::from_static("x-signature-timestamp");

/// An axum-style `TypedHeader` for the `x-signature-timestamp` header.
/// Example:
/// ```rs,no_run
/// fn handle(
///     TypedHeader(XSignatureTimestamp(delivery)): TypedHeader<XSignatureTimestamp>,
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
pub struct XSignatureTimestamp(pub String);

impl Header for XSignatureTimestamp {
    fn name() -> &'static HeaderName {
        &X_SIGNATURE_TIMESTAMP
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values
            .next()
            .and_then(|h| HeaderValue::to_str(h).ok())
            .map(|s| s.to_string())
            .ok_or_else(headers::Error::invalid)?;

        Ok(Self(value))
    }

    fn encode<E>(&self, _values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        // unnecessary, since we're only decoding
        unreachable!()
    }
}

/// The `x-signature-ed25519` header.
pub static X_SIGNATURE_ED25519: HeaderName = HeaderName::from_static("x-signature-ed25519");

/// An axum-style `TypedHeader` for the `x-signature-ed25519` header.
/// Example:
/// ```rs,no_run
/// fn handle(
///     TypedHeader(XSignatureEd25519(delivery)): TypedHeader<XSignatureEd25519>,
/// ) -> impl IntoResponse {
///     // ...
/// }
/// ```
pub struct XSignatureEd25519(pub String);

impl Header for XSignatureEd25519 {
    fn name() -> &'static HeaderName {
        &X_SIGNATURE_ED25519
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values
            .next()
            .and_then(|h| HeaderValue::to_str(h).ok())
            .map(|s| s.to_string())
            .ok_or_else(headers::Error::invalid)?;

        Ok(Self(value))
    }

    fn encode<E>(&self, _values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        // unnecessary, since we're only decoding
        unreachable!()
    }
}
