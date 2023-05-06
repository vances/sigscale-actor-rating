use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpServerReceiver, HttpServer, HttpRequest, HttpResponse, HeaderMap};
use wasmcloud_interface_numbergen::{NumberGenSender, NumberGen};
use serde::{Deserialize, Serialize};
use sigscale_interface_prefix::{PrefixTablesSender, PrefixTables, MatchPrefixRequest, MatchPrefixResponse};
// use wasmcloud_interface_logging::error;

#[derive(Actor, HealthResponder, Default, Debug)]
#[services(Actor, HttpServer)]
struct RatingActor {}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
#[allow(non_snake_case)]
struct RatingDataRequest {
	// mandatory attributes
	nfConsumerIdentification: NFIdentification,
	invocationTimeStamp: String,
	invocationSequenceNumber: u32,
	serviceRating: Vec<ServiceRatingRequest>,

	// optional attributes
	subscriptionId: Option<Vec<String>>,
	tenantIdentifier: Option<String>,
	mnSConsumerIdentifier: Option<String>,
	beginTimeStamp: Option<String>,
	oneTimeEvent: Option<bool>,
	oneTimeEventType: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
#[allow(non_snake_case)]
struct NFIdentification {
	// mandatory attributes
	nodeFunctionality: String,

	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	nFName: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	nFIPv4Address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	nFIPv6Address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	nFPLMNID: Option<PlmnId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	nFFqdn: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
struct PlmnId {
	// namdatory attributes
	mcc: String,
	mnc: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
#[allow(non_snake_case)]
struct ServiceRatingRequest {
	// optional attributes
	serviceId: Option<u32>,
	ratingGroup: Option<u32>,
	originationId: Option<Vec<OriginationId>>,
	destinationId: Option<Vec<DestinationId>>,
	serviceContextId: Option<String>,
//	serviceInformation: Option<ServiceInformation>,
//	counter: Option<Vec<Counter>>,
	basicPriceTimeStamp: Option<String>,
	requestSubType: Option<String>,
	requestedUnit: Option<RequestedUnit>,
	consumedUnit: Option<ConsumedUnit>,
	consumedUnitAfterTariffSwitch: Option<ConsumedUnit>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct OriginationId {
	// namdatory attributes
	originationIdType: String,
	originationIdData: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct DestinationId {
	// namdatory attributes
	destinationIdType: String,
	destinationIdData: String,
}

#[derive(Deserialize, Default, Debug)]
#[allow(non_snake_case)]
struct RequestedUnit {
	// optional attributes
	time: Option<u32>,
	totalVolume: Option<u64>,
	uplinkVolume: Option<u64>,
	downlinkVolume: Option<u64>,
	serviceSpecificUnit: Option<u64>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct GrantedUnit {
	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	time: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	totalVolume: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	uplinkVolume: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	downlinkVolume: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	serviceSpecificUnit: Option<u64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[allow(non_snake_case)]
struct ConsumedUnit {
	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	time: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	totalVolume: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	uplinkVolume: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	downlinkVolume: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	serviceSpecificUnit: Option<u64>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct RatingDataResponse {
	// mandatory attributes
	invocationTimeStamp: String,
	invocationSequenceNumber: u32,
	serviceRating: Vec<ServiceRatingResult>,

	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	invocationResult: Option<InvocationResult>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct ServiceRatingResult {
	// mandatory attributes
	resultCode: String,

	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	serviceId: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	ratingGroup: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	serviceContextId: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	grantedUnit: Option<GrantedUnit>,
	#[serde(skip_serializing_if = "Option::is_none")]
	consumedUnit: Option<ConsumedUnit>,
	#[serde(skip_serializing_if = "Option::is_none")]
	basicPrice: Option<Price>,
	#[serde(skip_serializing_if = "Option::is_none")]
	price: Option<Price>,
	#[serde(skip_serializing_if = "Option::is_none")]
	billingInfo: Option<String>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	counterPrice: Option<Vec<CounterPrice>>
//	#[serde(skip_serializing_if = "Option::is_none")]
//	impactOnCounter: Option<Vec<ImpactOnCounter>>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	tariffSwitchTime: Option<u32>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	currentTariff: Option<Tariff>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	nextTariff: Option<Tariff>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	expiryTime: Option<u32>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	validUnits: Option<u64>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	tariffAfterValidUnits: Option<Tariff,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	counterTariff: Option<Vec<CounterTariff>>,
//	#[serde(skip_serializing_if = "Option::is_none")]
//	requestedCounter: Option<Vec<u32>>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct Price {
	// mandatory attributes
	amount: UnitValue,

	// optional attributes
//	#[serde(skip_serializing_if = "Option::is_none")]
//	currencyCode: Option<CurrencyCode>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct UnitValue {
	// mandatory attributes
	valueDigits: u64,

	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	exponent: Option<i32>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct InvocationResult {
	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	error: Option<ProblemDetails>,
	#[serde(skip_serializing_if = "Option::is_none")]
	failureHandling: Option<String>,
}

#[derive(Serialize, Default, Debug)]
#[allow(non_snake_case)]
struct ProblemDetails {
	// mandatory attributes
	r#type: String,

	// optional attributes
	#[serde(skip_serializing_if = "Option::is_none")]
	title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	status: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	instance: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	cause: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	invalidParams: Option<Vec<InvalidParam>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	supportedFeatures: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	targetScp: Option<String>,
}

#[derive(Serialize, Default, Debug)]
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
			("POST", ["ratingdata"]) =>
				match serde_json::from_slice(&req.body) {
					Ok(rating_data_req) =>
						rating_start(ctx, rating_data_req).await,
					Err(error) =>
						// error!("serde_json::from_slice {}", error);
						Ok(HttpResponse::bad_request(error)),
				},
			("POST", ["ratingdata", _rating_data_ref, "update"]) =>
				match serde_json::from_slice(&req.body) {
					Ok(rating_data_req) =>
						rating_update(ctx, rating_data_req).await,
					Err(error) =>
						Ok(HttpResponse::bad_request(error)),
				},
			("POST", ["ratingdata", _ratingdata_ref, "release"]) =>
				match serde_json::from_slice(&req.body) {
					Ok(rating_data_req) =>
						rating_stop(ctx, rating_data_req).await,
					Err(error) =>
						Ok(HttpResponse::bad_request(error)),
				},
			("POST", _) =>
				Ok(HttpResponse::not_found()),
			(_, _) =>
				Ok(HttpResponse {
						status_code: 405,
						..Default::default()
				}),
		}
	}
}

async fn rating_start(ctx: &Context, rating_data_req: RatingDataRequest) -> RpcResult<HttpResponse> {
	let mut rating_data_res = RatingDataResponse {
		invocationTimeStamp: rating_data_req.invocationTimeStamp,
		invocationSequenceNumber: rating_data_req.invocationSequenceNumber,
		..Default::default()
	};
	let mut service_rating_res = match &rating_data_req.serviceRating {
		service_rating_req if service_rating_req.len() == 1 =>
			ServiceRatingResult {
					serviceId: service_rating_req[0].serviceId.clone(),
					ratingGroup: service_rating_req[0].ratingGroup.clone(),
					serviceContextId: service_rating_req[0].serviceContextId.clone(),
					..Default::default()
			},
		service_rating_req if service_rating_req.is_empty() =>
			return Ok(HttpResponse::bad_request("EmptyserviceRating")),
		service_rating_req if service_rating_req.len() > 1 =>
			return Ok(HttpResponse::bad_request("Too many serviceRating")),
		_service_rating_req =>
			return Ok(HttpResponse::bad_request("Missing serviceRating")),
	};
	let address = match &rating_data_req.serviceRating[0].destinationId {
		Some(addresses) if addresses.len() == 1 =>
			addresses[0].destinationIdData.to_string(),
		Some(addresses) if addresses.is_empty() =>
			return Ok(HttpResponse::bad_request("Missing destinationId")),
		Some(addresses) =>
			return Ok(HttpResponse::bad_request("Too many detinationId")),
		None =>
			return Ok(HttpResponse::bad_request("Missing serviceRating")),
	};
	let rating_data_ref: String = match NumberGenSender::new()
			.generate_guid(ctx)
			.await {
		Ok(guid) => guid,
		Err(error) => return Ok(HttpResponse::internal_server_error(error)),
	};
	let tariff = String::from("SMS");
	let prefix_provider = match PrefixTablesSender::new_with_link("Tariff") {
		Ok(provider) => provider,
		Err(error) => return Ok(HttpResponse::internal_server_error(error)),
	};
	match prefix_provider
			.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
			.await {
		Ok(MatchPrefixResponse{value: rate, ..}) => {
				let mut header = HeaderMap::new();
				let mut location = String::from("/ratingdata/");
				location.push_str(&rating_data_ref);
				header.insert("Location".to_string(), vec![location]);
				service_rating_res.resultCode = String::from("SUCCESS");
				service_rating_res.price = Some(Price {
					amount: UnitValue {
						valueDigits: rate,
						..Default::default()
					},
					..Default::default()
				});
				service_rating_res.consumedUnit = Some(ConsumedUnit {
					serviceSpecificUnit: Some(1),
					..Default::default()
				});
				rating_data_res.serviceRating.push(service_rating_res);
				Ok(HttpResponse {
						status_code: 201,
						header,
						body: serde_json::to_vec(&rating_data_res).unwrap()
				})
			},
		Err(error) =>
			Ok(HttpResponse::internal_server_error(error)),
	}
}

async fn rating_update(ctx: &Context, _rating_data_req: RatingDataRequest) -> RpcResult<HttpResponse> {
	let address = String::from("+14165551234");
	let tariff = String::from("SMS");
	let prefix_provider = match PrefixTablesSender::new_with_link("Tariff") {
		Ok(provider) => provider,
		Err(error) => return Ok(HttpResponse::internal_server_error(error)),
	};
	match prefix_provider
			.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
			.await {
			Ok(MatchPrefixResponse {value: _rate, ..}) =>
				Ok(HttpResponse {
						status_code: 200,
						// todo: format RatingDataResponse
						..Default::default()
				}),
			Err(error) =>
				Ok(HttpResponse::internal_server_error(error)),
	}
}

async fn rating_stop(ctx: &Context, _rating_data_req: RatingDataRequest) -> RpcResult<HttpResponse> {
	let address = String::from("+14165551234");
	let tariff = String::from("SMS");
	let prefix_provider = match PrefixTablesSender::new_with_link("Tariff") {
		Ok(provider) => provider,
		Err(error) => return Ok(HttpResponse::internal_server_error(error)),
	};
	match prefix_provider
			.match_prefix(ctx, &MatchPrefixRequest { name: tariff, address })
			.await {
			Ok(MatchPrefixResponse {value: _rate, ..}) =>
				Ok(HttpResponse {
						status_code: 200,
						// todo: format RatingDataResponse
						..Default::default()
				}),
			Err(error) =>
				Ok(HttpResponse::internal_server_error(error)),
	}
}

