use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct SpoNgEbObCaSeActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for SpoNgEbObCaSeActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(
        &self,
        _ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let spongebob_case = to_spongebob_case(&req.path);

        Ok(HttpResponse {
            body: spongebob_case.as_bytes().to_vec(),
            ..Default::default()
        })
    }
}

fn to_spongebob_case(source: &str) -> String {
    let mut idx = 0;
    source
        .to_string()
        .chars()
        .map(|c| {
            // Set odd numbered characters that are alphabetic to uppercase
            match (c.is_ascii_alphabetic(), idx % 2) {
                (true, 1) => {
                    idx += 1;
                    c.to_ascii_uppercase()
                }
                (true, 0) => {
                    idx += 1;
                    c
                }
                (_, _) => c,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::to_spongebob_case;

    #[test]
    fn it_works() {
        let sample_path = "honestly it compiled so it should work";
        let spongebob = to_spongebob_case(sample_path);
        assert_eq!(
            spongebob,
            "hOnEsTlY iT cOmPiLeD sO iT sHoUlD wOrK".to_string()
        );
    }
}
