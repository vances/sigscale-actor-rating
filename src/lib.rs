use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpServerReceiver, HttpServer, HttpRequest, HttpResponse, HeaderMap};
use wasmcloud_interface_numbergen::{NumberGenSender, NumberGen};
use serde_json::Value;
use sigscale_interface_prefix::{PrefixTablesSender, PrefixTables, MatchPrefixRequest, MatchPrefixResponse};
// use wasmcloud_interface_logging::error;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct RatingActor {}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl HttpServer for RatingActor {
	async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
		let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();
		match (req.method.as_ref(), trimmed_path.as_slice()) {
			("POST", ["ratingdata"]) => {
					match serde_json::from_slice::<Value>(&req.body) {
						Ok(rating_data_req) =>
							rating_start(ctx, &rating_data_req).await,
						Err(error) => {
							// error!("serde_json::from_slice {}", error);
							Ok(HttpResponse::bad_request(error))
						}
					}
				},
			("POST", ["ratingdata", _rating_data_ref, "update"]) => {
					match serde_json::from_slice::<Value>(&req.body) {
						Ok(rating_data_req) =>
							rating_update(ctx, &rating_data_req).await,
						Err(error) => {
							Ok(HttpResponse::bad_request(error))
						}
					}
				},
			("POST", ["ratingdata", _ratingdata_ref, "release"]) => {
					match serde_json::from_slice::<Value>(&req.body) {
						Ok(rating_data_req) =>
							rating_stop(ctx, &rating_data_req).await,
						Err(error) => {
							Ok(HttpResponse::bad_request(error))
						}
					}
				},
			("POST", _) => {
					Ok(HttpResponse::not_found())
				},
			(_, _) => {
					Ok(HttpResponse {
							status_code: 405,
							..Default::default()
					})
				},
		}
	}
}

async fn rating_start(ctx: &Context, rating_data_req: &Value) -> RpcResult<HttpResponse> {
	let address: String = "+14165551234".to_owned();
	match NumberGenSender::new()
			.generate_guid(ctx)
			.await {
		Ok(guid) => {
				let tariff: String = "SMS".to_owned();
				match PrefixTablesSender::new_with_link("Tariff") {
					Ok(prefix_provider) =>
						match prefix_provider
								.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
								.await {
							Ok(_) => {
									let mut header = HeaderMap::new();
									let mut location: String = "/ratingdata/".to_owned();
									location.push_str(&guid);
									header.insert("Location".to_string(), vec![location]);
									Ok(HttpResponse {
											status_code: 201,
											header,
											// todo: format RatingDataResponse
											body: serde_json::to_vec(&rating_data_req).unwrap(),
									})
								},
							Err(error) => {
								Ok(HttpResponse::internal_server_error(error))
							}
						},
					Err(error) => {
						Ok(HttpResponse::internal_server_error(error))
					},
				}
			},
		Err(error) => {
			Ok(HttpResponse::internal_server_error(error))
		},
	}
}

async fn rating_update(ctx: &Context, rating_data_req: &Value) -> RpcResult<HttpResponse> {
	let address: String = "+14165551234".to_owned();
	let tariff: String = "SMS".to_owned();
	match PrefixTablesSender::new_with_link("Tariff") {
		Ok(prefix_provider) => {
				match prefix_provider
						.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
						.await {
						Ok(_) => {
								Ok(HttpResponse {
										status_code: 200,
										// todo: format RatingDataResponse
										body: serde_json::to_vec(&rating_data_req).unwrap(),
										..Default::default()
								})
							},
						Err(error) => {
							Ok(HttpResponse::internal_server_error(error))
						},
				}
			},
		Err(error) => {
				Ok(HttpResponse::internal_server_error(error))
			},
	}
}

async fn rating_stop(ctx: &Context, rating_data_req: &Value) -> RpcResult<HttpResponse> {
	let address: String = "+14165551234".to_owned();
	let tariff: String = "SMS".to_owned();
	match PrefixTablesSender::new_with_link("Tariff") {
		Ok(prefix_provider) => {
				match prefix_provider
						.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
						.await {
						Ok(_) => {
								Ok(HttpResponse {
										status_code: 200,
										// todo: format RatingDataResponse
										body: serde_json::to_vec(&rating_data_req).unwrap(),
										..Default::default()
								})
							},
						Err(error) => {
							Ok(HttpResponse::internal_server_error(error))
						},
				}
			},
		Err(error) => {
				Ok(HttpResponse::internal_server_error(error))
			},
	}
}

