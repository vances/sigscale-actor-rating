use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpServerReceiver, HttpServer, HttpRequest, HttpResponse, HeaderMap};
use wasmcloud_interface_numbergen::{NumberGenSender, NumberGen};
use serde::{Deserialize, Serialize};
use sigscale_interface_prefix::{PrefixTablesSender, PrefixTables, MatchPrefixRequest, MatchPrefixResponse};
// use wasmcloud_interface_logging::error;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct RatingActor {}

#[derive(Deserialize)]struct RatingDataRequest {
	// mandatory attributes
	nfConsumerIdentification: NFIdentification,
	invocationTimeStamp: String,
	invocationSequenceNumber: u32,
	serviceRating: Vec<ServiceRatingRequest>,

	// optional attributes
	#[serde(default)]
	subscriptionId: Vec<String>,
	#[serde(default)]
	tenantIdentifier: String,
	#[serde(default)]
	mnSConsumerIdentifier: String,
	#[serde(default)]
	beginTimeStamp: String,
	#[serde(default)]
	oneTimeEvent: bool,
	#[serde(default)]
	oneTimeEventType: String,
}

#[derive(Deserialize)]
struct NFIdentification {
	// mandatory attributes
	nodeFunctionality: String,

	// optional attributes
	#[serde(default)]
	nFName: String,
	#[serde(default)]
	nFIPv4Address: String,
	#[serde(default)]
	nFIPv6Address: String,
	#[serde(default)]
	nFPLMNID: PlmnId,
	#[serde(default)]
	nFFqdn: String,
}

#[derive(Deserialize)]
#[serde(default)]
struct PlmnId {
	// namdatory attributes
	mcc: String,
	mnc: String,
}
impl Default for PlmnId {
	fn default() -> Self {
		PlmnId {
			mcc: "001".to_string(),
			mnc: "001".to_string(),
		}
	}
}

#[derive(Deserialize)]
struct ServiceRatingRequest {
	// optional attributes
	#[serde(default)]
	serviceId: u32,
	#[serde(default)]
	ratingGroup: u32,
	#[serde(default)]
	originationId: Vec<OriginationId>,
	#[serde(default)]
	destinationId: Vec<DestinationId>,
	#[serde(default)]
	serviceContextId: String,
//	#[serde(default)]
//	serviceInformation: ServiceInformation,
//	#[serde(default)]
//	counter: Vec<Counter>
	#[serde(default)]
	basicPriceTimeStamp: String,
	#[serde(default)]
	requestSubType: String,
	#[serde(default)]
	requestedUnit: RequestedUnit,
	#[serde(default)]
	consumedUnit: ConsumedUnit,
	#[serde(default)]
	consumedUnitAfterTariffSwitch: ConsumedUnit,
}

#[derive(Deserialize)]
struct OriginationId {
	// namdatory attributes
	originationIdType: String,
	originationIdData: String,
}

#[derive(Deserialize)]
struct DestinationId {
	// namdatory attributes
	destinationIdType: String,
	destinationIdData: String,
}

#[derive(Deserialize)]
#[derive(Default)]
#[serde(default)]
struct RequestedUnit {
	// optional attributes
	time: u32,
	totalVolume: u64,
	uplinkVolume: u64,
	downlinkVolume: u64,
	serviceSpecificUnit: u64,
}

#[derive(Serialize)]
#[serde(default)]
struct GrantedUnit {
	time: u32,
	totalVolume: u64,
	uplinkVolume: u64,
	downlinkVolume: u64,
	serviceSpecificUnit: u64,
}

#[derive(Deserialize)]
#[derive(Default)]
#[serde(default)]
struct ConsumedUnit {
	time: u32,
	totalVolume: u64,
	uplinkVolume: u64,
	downlinkVolume: u64,
	serviceSpecificUnit: u64,
}

#[derive(Serialize)]
struct RatingDataResponse {
	// mandatory attributes
	invocationTimeStamp: String,
	invocationSequenceNumber: u32,
	invocationResult: InvocationResult,
	serviceRating: Vec<ServiceRatingResult>,
}

#[derive(Serialize)]
struct ServiceRatingResult {
	// mandatory attributes
	resultCode: String,

	// optional attributes
	#[serde(default)]
	serviceId: u32,
	#[serde(default)]
	ratingGroup: u32,
	#[serde(default)]
	serviceContextId: String,
	#[serde(default)]
	grantedUnit: GrantedUnit,
	#[serde(default)]
	basicPrice: Price,
	#[serde(default)]
	price: Price,
	#[serde(default)]
	billingInfo: String,
//	#[serde(default)]
//	counterPrice: Vec<CounterPrice>
//	#[serde(default)]
//	impactOnCounter: Vec<ImpactOnCounter>,
//	#[serde(default)]
//	tariffSwitchTime: u32,
//	#[serde(default)]
//	currentTariff: Tariff,
//	#[serde(default)]
//	nextTariff: Tariff,
//	#[serde(default)]
//	expiryTime: u32,
//	#[serde(default)]
//	validUnits: u64,
//	#[serde(default)]
//	tariffAfterValidUnits: Tariff,
//	#[serde(default)]
//	counterTariff: Vec<CounterTariff>,
//	#[serde(default)]
//	requestedCounter: Vec<u32>,
}

#[derive(Serialize)]
struct Price {
	// mandatory attributes
	amount: UnitValue,

	// optional attributes
//	#[serde(default)]
//	currencyCode: CurrencyCode,
}

#[derive(Serialize)]
struct UnitValue {
	// mandatory attributes
	valueDigits: u64,

	// optional attributes
	#[serde(default)]
	exponent: i32,
}

#[derive(Serialize)]
struct InvocationResult {
	// optional attributes
	#[serde(default)]
	error: ProblemDetails,
	#[serde(default)]
	failureHandling: String,
}

#[derive(Serialize)]
struct ProblemDetails {
	// mandatory attributes
	r#type: String,

	// optional attributes
	#[serde(default)]
	title: String,
	#[serde(default)]
	status: u16,
	#[serde(default)]
	detail: String,
	#[serde(default)]
	instance: String,
	#[serde(default)]
	cause: String,
	#[serde(default)]
	invalidParams: Vec<InvalidParam>,
	#[serde(default)]
	supportedFeatures: String,
	#[serde(default)]
	targetScp: String,
}

#[derive(Serialize)]
struct InvalidParam {
	// mandatory attributes
	param: String,
	reason: String,
}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl HttpServer for RatingActor {
	async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
		let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();
		match (req.method.as_ref(), trimmed_path.as_slice()) {
			("POST", ["ratingdata"]) => {
					match serde_json::from_slice(&req.body) {
						Ok(rating_data_req) =>
							rating_start(ctx, rating_data_req).await,
						Err(error) => {
							// error!("serde_json::from_slice {}", error);
							Ok(HttpResponse::bad_request(error))
						}
					}
				},
			("POST", ["ratingdata", _rating_data_ref, "update"]) => {
					match serde_json::from_slice(&req.body) {
						Ok(rating_data_req) =>
							rating_update(ctx, rating_data_req).await,
						Err(error) => {
							Ok(HttpResponse::bad_request(error))
						}
					}
				},
			("POST", ["ratingdata", _ratingdata_ref, "release"]) => {
					match serde_json::from_slice(&req.body) {
						Ok(rating_data_req) =>
							rating_stop(ctx, rating_data_req).await,
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

async fn rating_start(ctx: &Context, rating_data_req: RatingDataRequest) -> RpcResult<HttpResponse> {
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
							Ok(MatchPrefixResponse{value: _rate, ..}) => {
									let mut header = HeaderMap::new();
									let mut location: String = "/ratingdata/".to_owned();
									location.push_str(&guid);
									header.insert("Location".to_string(), vec![location]);
									Ok(HttpResponse {
											status_code: 201,
											header,
											// todo: format RatingDataResponse
											// body: serde_json::to_vec(&rating_data_req).unwrap(),
											..Default::default()
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

async fn rating_update(ctx: &Context, rating_data_req: RatingDataRequest) -> RpcResult<HttpResponse> {
	let address: String = "+14165551234".to_owned();
	let tariff: String = "SMS".to_owned();
	match PrefixTablesSender::new_with_link("Tariff") {
		Ok(prefix_provider) => {
				match prefix_provider
						.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
						.await {
						Ok(MatchPrefixResponse {value: _rate, ..}) => {
								Ok(HttpResponse {
										status_code: 200,
										// todo: format RatingDataResponse
										// body: serde_json::to_vec(&rating_data_req).unwrap(),
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

async fn rating_stop(ctx: &Context, rating_data_req: RatingDataRequest) -> RpcResult<HttpResponse> {
	let address: String = "+14165551234".to_owned();
	let tariff: String = "SMS".to_owned();
	match PrefixTablesSender::new_with_link("Tariff") {
		Ok(prefix_provider) => {
				match prefix_provider
						.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
						.await {
						Ok(MatchPrefixResponse {value: _rate, ..}) => {
								Ok(HttpResponse {
										status_code: 200,
										// todo: format RatingDataResponse
										// body: serde_json::to_vec(&rating_data_req).unwrap(),
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

