# Rating Actor
This actor makes use of three capabilities:
- `wasmcloud:httpserver`
- `wasmcloud:builtin:numbergen`
- `sigscale:prefixtables`

## The implementation
To respond to http requests, the actor must implement the
`HttpResponse` method of the
[HttpServer interface](https://github.com/wasmCloud/interfaces/tree/main/httpserver) interface.
The semantics of this REST API is defined in the Nrf
[OAS](https://app.swaggerhub.com/apis/SigScale/nrf-rating/1.0.0).
To determine a rate for charging service usage received in Nrf requests
the Actor must implement the `MatchPrefixRequest` method of the
Prefix Tables interface (`sigscale:prefixtables`).

The implementation is in the file [src/lib.rs](./src/lib.rs)

## See it in action

- To compile the actor and generate a signed Webassembly module, type `wash build`.
- Run the wasmCloud host with `wash up`
- In your browser at `localhost:4000`, in the **Actors** table, use the dropdown to start the actor **From File** and select the built and signed module from step 1
- Start the HTTP server provider **From Registry** with OCI reference `wasmcloud.azurecr.io/httpserver:0.17.0`
- Link your Rating actor to the HTTP server provider with a link value of `address=0.0.0.0:8080`
- Start the Prefix Tables provider **From File** with `../prefixtables-provider/build/prefixtables_provider.par.gz`
- Run the `rate.sh` shell script to send an Nrf request

### In a browser

Visit the url "http://localhost:8000" to see your response in the browser.

# Detailed Instructions

## Start the wasmCloud Host
	wash up

## Get the Host ID
	wash ctl get hosts

## Get inventory on the Host
	wash ctl get inventory NBFQBFYA7AFZIOGZTSR5EAJGQWC4TANWCURJD4RMZT2WMH547XVZTQ6W

## Start Rating Actor
	wash ctl start actor file:///Users/vances/rating/build/rating_s.wasm

## Start HTTP Server Capability Provider
	wash ctl start provider --link-name Nrf wasmcloud.azurecr.io/httpserver:0.17.0

## Add Nrf Link Definition
	wash ctl link put --link-name Nrf MD44U6Y2DXLMRL2GUWSK5SRBXQ3KPPEUSNNIK6XY4BJUEPXQWHI5NERX VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M  wasmcloud:httpserver

## Start Prefix Tables Capability Provider (fails)
	wash ctl start provider --link-name Tariff file:///Users/vances/prefixtables-provider/build/prefixtables_provider.par.gz

## Add Tariff Link Definition
	wash ctl link put --link-name Tariff MD44U6Y2DXLMRL2GUWSK5SRBXQ3KPPEUSNNIK6XY4BJUEPXQWHI5NERX VCJD5DQJVK4OBNQ7LIIMIUXZUGIH72LSEDCQKWF26FGELFOVIKZ53KNX sigscale:prefixtables

## Connect to the wasmcloud_host node
	erl -sname $LOGNAME -setcookie GYSX7CS5XWALTBRM4CKFLFLIB2OM5ARSMIG4L52XZQH22GEPQEJQ==== -remsh wasmcloud_host

