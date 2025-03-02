// Code generated by gRPC proto compiler.  DO NOT EDIT!
// source: platform.proto

#import <Foundation/Foundation.h>

#if !defined(GPB_GRPC_FORWARD_DECLARE_MESSAGE_PROTO) || !GPB_GRPC_FORWARD_DECLARE_MESSAGE_PROTO
#import "Platform.pbobjc.h"
#endif

#if !defined(GPB_GRPC_PROTOCOL_ONLY) || !GPB_GRPC_PROTOCOL_ONLY
#import <ProtoRPC/ProtoService.h>
#import <ProtoRPC/ProtoRPCLegacy.h>
#import <RxLibrary/GRXWriteable.h>
#import <RxLibrary/GRXWriter.h>
#endif

@class BroadcastStateTransitionRequest;
@class BroadcastStateTransitionResponse;
@class GetConsensusParamsRequest;
@class GetConsensusParamsResponse;
@class GetDataContractHistoryRequest;
@class GetDataContractHistoryResponse;
@class GetDataContractRequest;
@class GetDataContractResponse;
@class GetDataContractsRequest;
@class GetDataContractsResponse;
@class GetDocumentsRequest;
@class GetDocumentsResponse;
@class GetIdentitiesByPublicKeyHashesRequest;
@class GetIdentitiesByPublicKeyHashesResponse;
@class GetIdentitiesRequest;
@class GetIdentitiesResponse;
@class GetIdentityBalanceAndRevisionResponse;
@class GetIdentityBalanceResponse;
@class GetIdentityByPublicKeyHashesRequest;
@class GetIdentityByPublicKeyHashesResponse;
@class GetIdentityKeysRequest;
@class GetIdentityKeysResponse;
@class GetIdentityRequest;
@class GetIdentityResponse;
@class GetProofsRequest;
@class GetProofsResponse;
@class WaitForStateTransitionResultRequest;
@class WaitForStateTransitionResultResponse;

#if !defined(GPB_GRPC_FORWARD_DECLARE_MESSAGE_PROTO) || !GPB_GRPC_FORWARD_DECLARE_MESSAGE_PROTO
#if defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS) && GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
  #import <Protobuf/GPBWrappers.pbobjc.h>
#else
  #import "GPBWrappers.pbobjc.h"
#endif
#if defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS) && GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
  #import <Protobuf/GPBStruct.pbobjc.h>
#else
  #import "GPBStruct.pbobjc.h"
#endif
#if defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS) && GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
  #import <Protobuf/GPBTimestamp.pbobjc.h>
#else
  #import "GPBTimestamp.pbobjc.h"
#endif
#endif

@class GRPCUnaryProtoCall;
@class GRPCStreamingProtoCall;
@class GRPCCallOptions;
@protocol GRPCProtoResponseHandler;
@class GRPCProtoCall;


NS_ASSUME_NONNULL_BEGIN

@protocol Platform2 <NSObject>

#pragma mark broadcastStateTransition(BroadcastStateTransitionRequest) returns (BroadcastStateTransitionResponse)

- (GRPCUnaryProtoCall *)broadcastStateTransitionWithMessage:(BroadcastStateTransitionRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentity(GetIdentityRequest) returns (GetIdentityResponse)

- (GRPCUnaryProtoCall *)getIdentityWithMessage:(GetIdentityRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentities(GetIdentitiesRequest) returns (GetIdentitiesResponse)

- (GRPCUnaryProtoCall *)getIdentitiesWithMessage:(GetIdentitiesRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentityKeys(GetIdentityKeysRequest) returns (GetIdentityKeysResponse)

- (GRPCUnaryProtoCall *)getIdentityKeysWithMessage:(GetIdentityKeysRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentityBalance(GetIdentityRequest) returns (GetIdentityBalanceResponse)

/**
 * rpc getIdentitiesKeys (GetIdentitiesKeysRequest) returns (GetIdentitiesKeysResponse);
 */
- (GRPCUnaryProtoCall *)getIdentityBalanceWithMessage:(GetIdentityRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentityBalanceAndRevision(GetIdentityRequest) returns (GetIdentityBalanceAndRevisionResponse)

- (GRPCUnaryProtoCall *)getIdentityBalanceAndRevisionWithMessage:(GetIdentityRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getProofs(GetProofsRequest) returns (GetProofsResponse)

- (GRPCUnaryProtoCall *)getProofsWithMessage:(GetProofsRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getDataContract(GetDataContractRequest) returns (GetDataContractResponse)

- (GRPCUnaryProtoCall *)getDataContractWithMessage:(GetDataContractRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getDataContractHistory(GetDataContractHistoryRequest) returns (GetDataContractHistoryResponse)

- (GRPCUnaryProtoCall *)getDataContractHistoryWithMessage:(GetDataContractHistoryRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getDataContracts(GetDataContractsRequest) returns (GetDataContractsResponse)

- (GRPCUnaryProtoCall *)getDataContractsWithMessage:(GetDataContractsRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getDocuments(GetDocumentsRequest) returns (GetDocumentsResponse)

- (GRPCUnaryProtoCall *)getDocumentsWithMessage:(GetDocumentsRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentitiesByPublicKeyHashes(GetIdentitiesByPublicKeyHashesRequest) returns (GetIdentitiesByPublicKeyHashesResponse)

- (GRPCUnaryProtoCall *)getIdentitiesByPublicKeyHashesWithMessage:(GetIdentitiesByPublicKeyHashesRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getIdentityByPublicKeyHashes(GetIdentityByPublicKeyHashesRequest) returns (GetIdentityByPublicKeyHashesResponse)

- (GRPCUnaryProtoCall *)getIdentityByPublicKeyHashesWithMessage:(GetIdentityByPublicKeyHashesRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark waitForStateTransitionResult(WaitForStateTransitionResultRequest) returns (WaitForStateTransitionResultResponse)

- (GRPCUnaryProtoCall *)waitForStateTransitionResultWithMessage:(WaitForStateTransitionResultRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

#pragma mark getConsensusParams(GetConsensusParamsRequest) returns (GetConsensusParamsResponse)

- (GRPCUnaryProtoCall *)getConsensusParamsWithMessage:(GetConsensusParamsRequest *)message responseHandler:(id<GRPCProtoResponseHandler>)handler callOptions:(GRPCCallOptions *_Nullable)callOptions;

@end

/**
 * The methods in this protocol belong to a set of old APIs that have been deprecated. They do not
 * recognize call options provided in the initializer. Using the v2 protocol is recommended.
 */
@protocol Platform <NSObject>

#pragma mark broadcastStateTransition(BroadcastStateTransitionRequest) returns (BroadcastStateTransitionResponse)

- (void)broadcastStateTransitionWithRequest:(BroadcastStateTransitionRequest *)request handler:(void(^)(BroadcastStateTransitionResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTobroadcastStateTransitionWithRequest:(BroadcastStateTransitionRequest *)request handler:(void(^)(BroadcastStateTransitionResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentity(GetIdentityRequest) returns (GetIdentityResponse)

- (void)getIdentityWithRequest:(GetIdentityRequest *)request handler:(void(^)(GetIdentityResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetIdentityWithRequest:(GetIdentityRequest *)request handler:(void(^)(GetIdentityResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentities(GetIdentitiesRequest) returns (GetIdentitiesResponse)

- (void)getIdentitiesWithRequest:(GetIdentitiesRequest *)request handler:(void(^)(GetIdentitiesResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetIdentitiesWithRequest:(GetIdentitiesRequest *)request handler:(void(^)(GetIdentitiesResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentityKeys(GetIdentityKeysRequest) returns (GetIdentityKeysResponse)

- (void)getIdentityKeysWithRequest:(GetIdentityKeysRequest *)request handler:(void(^)(GetIdentityKeysResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetIdentityKeysWithRequest:(GetIdentityKeysRequest *)request handler:(void(^)(GetIdentityKeysResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentityBalance(GetIdentityRequest) returns (GetIdentityBalanceResponse)

/**
 * rpc getIdentitiesKeys (GetIdentitiesKeysRequest) returns (GetIdentitiesKeysResponse);
 *
 * This method belongs to a set of APIs that have been deprecated. Using the v2 API is recommended.
 */
- (void)getIdentityBalanceWithRequest:(GetIdentityRequest *)request handler:(void(^)(GetIdentityBalanceResponse *_Nullable response, NSError *_Nullable error))handler;

/**
 * rpc getIdentitiesKeys (GetIdentitiesKeysRequest) returns (GetIdentitiesKeysResponse);
 *
 * This method belongs to a set of APIs that have been deprecated. Using the v2 API is recommended.
 */
- (GRPCProtoCall *)RPCTogetIdentityBalanceWithRequest:(GetIdentityRequest *)request handler:(void(^)(GetIdentityBalanceResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentityBalanceAndRevision(GetIdentityRequest) returns (GetIdentityBalanceAndRevisionResponse)

- (void)getIdentityBalanceAndRevisionWithRequest:(GetIdentityRequest *)request handler:(void(^)(GetIdentityBalanceAndRevisionResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetIdentityBalanceAndRevisionWithRequest:(GetIdentityRequest *)request handler:(void(^)(GetIdentityBalanceAndRevisionResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getProofs(GetProofsRequest) returns (GetProofsResponse)

- (void)getProofsWithRequest:(GetProofsRequest *)request handler:(void(^)(GetProofsResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetProofsWithRequest:(GetProofsRequest *)request handler:(void(^)(GetProofsResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getDataContract(GetDataContractRequest) returns (GetDataContractResponse)

- (void)getDataContractWithRequest:(GetDataContractRequest *)request handler:(void(^)(GetDataContractResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetDataContractWithRequest:(GetDataContractRequest *)request handler:(void(^)(GetDataContractResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getDataContractHistory(GetDataContractHistoryRequest) returns (GetDataContractHistoryResponse)

- (void)getDataContractHistoryWithRequest:(GetDataContractHistoryRequest *)request handler:(void(^)(GetDataContractHistoryResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetDataContractHistoryWithRequest:(GetDataContractHistoryRequest *)request handler:(void(^)(GetDataContractHistoryResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getDataContracts(GetDataContractsRequest) returns (GetDataContractsResponse)

- (void)getDataContractsWithRequest:(GetDataContractsRequest *)request handler:(void(^)(GetDataContractsResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetDataContractsWithRequest:(GetDataContractsRequest *)request handler:(void(^)(GetDataContractsResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getDocuments(GetDocumentsRequest) returns (GetDocumentsResponse)

- (void)getDocumentsWithRequest:(GetDocumentsRequest *)request handler:(void(^)(GetDocumentsResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetDocumentsWithRequest:(GetDocumentsRequest *)request handler:(void(^)(GetDocumentsResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentitiesByPublicKeyHashes(GetIdentitiesByPublicKeyHashesRequest) returns (GetIdentitiesByPublicKeyHashesResponse)

- (void)getIdentitiesByPublicKeyHashesWithRequest:(GetIdentitiesByPublicKeyHashesRequest *)request handler:(void(^)(GetIdentitiesByPublicKeyHashesResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetIdentitiesByPublicKeyHashesWithRequest:(GetIdentitiesByPublicKeyHashesRequest *)request handler:(void(^)(GetIdentitiesByPublicKeyHashesResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getIdentityByPublicKeyHashes(GetIdentityByPublicKeyHashesRequest) returns (GetIdentityByPublicKeyHashesResponse)

- (void)getIdentityByPublicKeyHashesWithRequest:(GetIdentityByPublicKeyHashesRequest *)request handler:(void(^)(GetIdentityByPublicKeyHashesResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetIdentityByPublicKeyHashesWithRequest:(GetIdentityByPublicKeyHashesRequest *)request handler:(void(^)(GetIdentityByPublicKeyHashesResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark waitForStateTransitionResult(WaitForStateTransitionResultRequest) returns (WaitForStateTransitionResultResponse)

- (void)waitForStateTransitionResultWithRequest:(WaitForStateTransitionResultRequest *)request handler:(void(^)(WaitForStateTransitionResultResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTowaitForStateTransitionResultWithRequest:(WaitForStateTransitionResultRequest *)request handler:(void(^)(WaitForStateTransitionResultResponse *_Nullable response, NSError *_Nullable error))handler;


#pragma mark getConsensusParams(GetConsensusParamsRequest) returns (GetConsensusParamsResponse)

- (void)getConsensusParamsWithRequest:(GetConsensusParamsRequest *)request handler:(void(^)(GetConsensusParamsResponse *_Nullable response, NSError *_Nullable error))handler;

- (GRPCProtoCall *)RPCTogetConsensusParamsWithRequest:(GetConsensusParamsRequest *)request handler:(void(^)(GetConsensusParamsResponse *_Nullable response, NSError *_Nullable error))handler;


@end


#if !defined(GPB_GRPC_PROTOCOL_ONLY) || !GPB_GRPC_PROTOCOL_ONLY
/**
 * Basic service implementation, over gRPC, that only does
 * marshalling and parsing.
 */
@interface Platform : GRPCProtoService<Platform2, Platform>
- (instancetype)initWithHost:(NSString *)host callOptions:(GRPCCallOptions *_Nullable)callOptions NS_DESIGNATED_INITIALIZER;
+ (instancetype)serviceWithHost:(NSString *)host callOptions:(GRPCCallOptions *_Nullable)callOptions;
// The following methods belong to a set of old APIs that have been deprecated.
- (instancetype)initWithHost:(NSString *)host;
+ (instancetype)serviceWithHost:(NSString *)host;
@end
#endif

NS_ASSUME_NONNULL_END

