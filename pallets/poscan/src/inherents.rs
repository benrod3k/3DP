use codec::Decode;
use sp_inherents::{InherentIdentifier, InherentData};

// This needs to be unique for the runtime.
const INHERENT_IDENTIFIER: InherentIdentifier = *b"p3d";

/// Some custom inherent data provider
struct InherentDataProvider;

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(
        &self,
        inherent_data: &mut InherentData,
    ) -> Result<(), sp_inherents::Error> {
        // We can insert any data that implements [`codec::Encode`].
        inherent_data.put_data(INHERENT_IDENTIFIER, &"hello")
    }

    /// When validating the inherents, the runtime implementation can throw errors. We support
    /// two error modes, fatal and non-fatal errors. A fatal error means that the block is invalid
    /// and this function here should return `Err(_)` to not import the block. Non-fatal errors
    /// are allowed to be handled here in this function and the function should return `Ok(())`
    /// if it could be handled. A non-fatal error is for example that a block is in the future
    /// from the point of view of the local node. In such a case the block import for example
    /// should be delayed until the block is valid.
    ///
    /// If this functions returns `None`, it means that it is not responsible for this error or
    /// that the error could not be interpreted.
    async fn try_handle_error(
        &self,
        identifier: &InherentIdentifier,
        mut error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        // Check if this error belongs to us.
        if *identifier != INHERENT_IDENTIFIER {
            return None;
        }

        // For demonstration purposes we are using a `String` as error type. In real
        // implementations it is advised to not use `String`.
        Some(Err(
            sp_inherents::Error::Application(Box::from(String::decode(&mut error).ok()?))
        ))
    }
}