mod middleware;
mod server_api;
mod server_api_mock;

pub use server_api::*;
// TODO: ignore mock files in production
use crate::errors::DocError;
use backend_service::configuration::ClientServerConfiguration;
use flowy_document_infra::entities::doc::{CreateDocParams, Doc, DocIdentifier, UpdateDocParams};
use lib_infra::future::ResultFuture;
pub use server_api_mock::*;
use std::sync::Arc;

pub(crate) type Server = Arc<dyn DocumentServerAPI + Send + Sync>;
pub trait DocumentServerAPI {
    fn create_doc(&self, token: &str, params: CreateDocParams) -> ResultFuture<(), DocError>;

    fn read_doc(&self, token: &str, params: DocIdentifier) -> ResultFuture<Option<Doc>, DocError>;

    fn update_doc(&self, token: &str, params: UpdateDocParams) -> ResultFuture<(), DocError>;
}

pub(crate) fn construct_doc_server(
    server_config: &ClientServerConfiguration,
) -> Arc<dyn DocumentServerAPI + Send + Sync> {
    if cfg!(feature = "http_server") {
        Arc::new(DocServer::new(server_config.clone()))
    } else {
        Arc::new(DocServerMock {})
    }
}
