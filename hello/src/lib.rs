use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

use wasmcloud_examples_payments::{Payments, AuthorizePaymentRequest, PaymentsSender};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for HelloActor {

    async fn handle_request(
        &self,
        ctx: &Context,
        _req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let sender = PaymentsSender::new();
        let result = sender.authorize_payment(
            ctx,
            &AuthorizePaymentRequest {
                amount: 1,
                payment_entity: "pe".to_owned(),
                payment_method: "pm".to_owned(),
                reference_id: "ref".to_owned(),
                tax: 42,
            },
        ).await;
        match result {
            Ok(_) => Ok(HttpResponse {
                body: "ok".as_bytes().to_vec(),
                ..Default::default()
            }),
            Err(err) => Ok(HttpResponse {
                body: format!("{:?}", err).as_bytes().to_vec(),
                status_code: 500 as u16,
                ..Default::default()
            }),
        }
    }
}
