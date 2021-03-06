//! The WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Command-line interface for the WASM language server.
pub mod cli;

// Core functionality for the WASM language server.
mod core;

// Functionality related to implementation of the Language Server Protocol (LSP).
pub mod lsp;

// Definitions related to the wasm-language-server crate.
mod package;

// Services (components) of the WASM language server.
mod service;

// Various utility functionality, e.g., for handling lsp or tree-sitter data.
mod util;

#[cfg(feature = "test")]
#[doc(hidden)]
pub mod test {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! assert_ready {
        ($service:expr, $status:expr) => {
            assert_eq!($service.poll_ready(), Poll::Ready($status));
        };
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! assert_exchange {
        ($service:expr, $request:expr, $response:expr) => {
            assert_eq!(test::service::call($service, $request).await, $response);
        };
    }

    pub mod service {
        use serde_json::Value;
        use tower_lsp::{ExitedError, LspService, MessageStream};
        use tower_test::mock::Spawn;

        pub async fn call(service: &mut Spawn<LspService>, request: &Value) -> Result<Option<Value>, ExitedError> {
            let request = serde_json::from_value(request.clone()).unwrap();
            let response = service.call(request).await?;
            let response = response.and_then(|x| serde_json::to_value(x).ok());
            Ok(response)
        }

        pub fn spawn() -> anyhow::Result<(Spawn<LspService>, MessageStream)> {
            let (service, messages) = LspService::new(|client| {
                let server = crate::lsp::server::Server::new(client);
                server.unwrap()
            });
            Ok((Spawn::new(service), messages))
        }
    }
}
