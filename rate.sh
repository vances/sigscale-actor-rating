#!/bin/sh

ISO8601=`date -Iseconds`
TIMESTAMP="\"invocationTimeStamp\":\"${ISO8601}\""
SEQUENCE="\"invocationSequenceNumber\": ${RANDOM}"
CONSUMER="\"nfConsumerIdentification\":{\"nodeFunctionality\":\"OCF\"}"
SUBSCRIBER="\"subscriptionId\":[\"msisdn-14165551234\",\"imsi-001001000000001\"]"
EVENT="\"oneTimeEvent\":true"
EVENTTYPE="\"oneTimeEventType\":\"IEC\""
CONTEXT="\"serviceContextId\":\"32274@3gpp.org\""
SERVICE="\"serviceId\":4"
DESTINATION="\"destinationId\":[{\"destinationIdType\":\"DN\",\"destinationIdData\":\"14165556789\"}]"
REQTYPE="\"requestSubType\":\"DEBIT\""
SERVICERATING="\"serviceRating\":[{$CONTEXT,$SERVICE,$DESTINATION,$REQTYPE}]"
DATA="{$TIMESTAMP,$SEQUENCE,$CONSUMER,$SUBSCRIBER,$EVENT,$EVENTTYPE,$SERVICERATING}"

curl -s -H "Content-Type: application/json" \
		-H "Accept: application/json, application/problem+json" \
		--data "${DATA}" \
		http://localhost:8000/ratingdata
