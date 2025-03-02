// package: org.dash.platform.dapi.v0
// file: platform.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_wrappers_pb from "google-protobuf/google/protobuf/wrappers_pb";
import * as google_protobuf_struct_pb from "google-protobuf/google/protobuf/struct_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class Proof extends jspb.Message {
  getGrovedbProof(): Uint8Array | string;
  getGrovedbProof_asU8(): Uint8Array;
  getGrovedbProof_asB64(): string;
  setGrovedbProof(value: Uint8Array | string): void;

  getQuorumHash(): Uint8Array | string;
  getQuorumHash_asU8(): Uint8Array;
  getQuorumHash_asB64(): string;
  setQuorumHash(value: Uint8Array | string): void;

  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  getRound(): number;
  setRound(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Proof.AsObject;
  static toObject(includeInstance: boolean, msg: Proof): Proof.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Proof, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Proof;
  static deserializeBinaryFromReader(message: Proof, reader: jspb.BinaryReader): Proof;
}

export namespace Proof {
  export type AsObject = {
    grovedbProof: Uint8Array | string,
    quorumHash: Uint8Array | string,
    signature: Uint8Array | string,
    round: number,
  }
}

export class ResponseMetadata extends jspb.Message {
  getHeight(): number;
  setHeight(value: number): void;

  getCoreChainLockedHeight(): number;
  setCoreChainLockedHeight(value: number): void;

  getTimeMs(): number;
  setTimeMs(value: number): void;

  getProtocolVersion(): number;
  setProtocolVersion(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ResponseMetadata.AsObject;
  static toObject(includeInstance: boolean, msg: ResponseMetadata): ResponseMetadata.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ResponseMetadata, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ResponseMetadata;
  static deserializeBinaryFromReader(message: ResponseMetadata, reader: jspb.BinaryReader): ResponseMetadata;
}

export namespace ResponseMetadata {
  export type AsObject = {
    height: number,
    coreChainLockedHeight: number,
    timeMs: number,
    protocolVersion: number,
  }
}

export class StateTransitionBroadcastError extends jspb.Message {
  getCode(): number;
  setCode(value: number): void;

  getMessage(): string;
  setMessage(value: string): void;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StateTransitionBroadcastError.AsObject;
  static toObject(includeInstance: boolean, msg: StateTransitionBroadcastError): StateTransitionBroadcastError.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StateTransitionBroadcastError, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StateTransitionBroadcastError;
  static deserializeBinaryFromReader(message: StateTransitionBroadcastError, reader: jspb.BinaryReader): StateTransitionBroadcastError;
}

export namespace StateTransitionBroadcastError {
  export type AsObject = {
    code: number,
    message: string,
    data: Uint8Array | string,
  }
}

export class BroadcastStateTransitionRequest extends jspb.Message {
  getStateTransition(): Uint8Array | string;
  getStateTransition_asU8(): Uint8Array;
  getStateTransition_asB64(): string;
  setStateTransition(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BroadcastStateTransitionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BroadcastStateTransitionRequest): BroadcastStateTransitionRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: BroadcastStateTransitionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BroadcastStateTransitionRequest;
  static deserializeBinaryFromReader(message: BroadcastStateTransitionRequest, reader: jspb.BinaryReader): BroadcastStateTransitionRequest;
}

export namespace BroadcastStateTransitionRequest {
  export type AsObject = {
    stateTransition: Uint8Array | string,
  }
}

export class BroadcastStateTransitionResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BroadcastStateTransitionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: BroadcastStateTransitionResponse): BroadcastStateTransitionResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: BroadcastStateTransitionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BroadcastStateTransitionResponse;
  static deserializeBinaryFromReader(message: BroadcastStateTransitionResponse, reader: jspb.BinaryReader): BroadcastStateTransitionResponse;
}

export namespace BroadcastStateTransitionResponse {
  export type AsObject = {
  }
}

export class GetIdentityRequest extends jspb.Message {
  getId(): Uint8Array | string;
  getId_asU8(): Uint8Array;
  getId_asB64(): string;
  setId(value: Uint8Array | string): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityRequest): GetIdentityRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityRequest;
  static deserializeBinaryFromReader(message: GetIdentityRequest, reader: jspb.BinaryReader): GetIdentityRequest;
}

export namespace GetIdentityRequest {
  export type AsObject = {
    id: Uint8Array | string,
    prove: boolean,
  }
}

export class GetIdentityResponse extends jspb.Message {
  hasIdentity(): boolean;
  clearIdentity(): void;
  getIdentity(): Uint8Array | string;
  getIdentity_asU8(): Uint8Array;
  getIdentity_asB64(): string;
  setIdentity(value: Uint8Array | string): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentityResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityResponse): GetIdentityResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityResponse;
  static deserializeBinaryFromReader(message: GetIdentityResponse, reader: jspb.BinaryReader): GetIdentityResponse;
}

export namespace GetIdentityResponse {
  export type AsObject = {
    identity: Uint8Array | string,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    IDENTITY = 1,
    PROOF = 2,
  }
}

export class GetIdentitiesRequest extends jspb.Message {
  clearIdsList(): void;
  getIdsList(): Array<Uint8Array | string>;
  getIdsList_asU8(): Array<Uint8Array>;
  getIdsList_asB64(): Array<string>;
  setIdsList(value: Array<Uint8Array | string>): void;
  addIds(value: Uint8Array | string, index?: number): Uint8Array | string;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentitiesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentitiesRequest): GetIdentitiesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentitiesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentitiesRequest;
  static deserializeBinaryFromReader(message: GetIdentitiesRequest, reader: jspb.BinaryReader): GetIdentitiesRequest;
}

export namespace GetIdentitiesRequest {
  export type AsObject = {
    idsList: Array<Uint8Array | string>,
    prove: boolean,
  }
}

export class GetIdentitiesResponse extends jspb.Message {
  hasIdentities(): boolean;
  clearIdentities(): void;
  getIdentities(): GetIdentitiesResponse.Identities | undefined;
  setIdentities(value?: GetIdentitiesResponse.Identities): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentitiesResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentitiesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentitiesResponse): GetIdentitiesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentitiesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentitiesResponse;
  static deserializeBinaryFromReader(message: GetIdentitiesResponse, reader: jspb.BinaryReader): GetIdentitiesResponse;
}

export namespace GetIdentitiesResponse {
  export type AsObject = {
    identities?: GetIdentitiesResponse.Identities.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class IdentityValue extends jspb.Message {
    getValue(): Uint8Array | string;
    getValue_asU8(): Uint8Array;
    getValue_asB64(): string;
    setValue(value: Uint8Array | string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): IdentityValue.AsObject;
    static toObject(includeInstance: boolean, msg: IdentityValue): IdentityValue.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: IdentityValue, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): IdentityValue;
    static deserializeBinaryFromReader(message: IdentityValue, reader: jspb.BinaryReader): IdentityValue;
  }

  export namespace IdentityValue {
    export type AsObject = {
      value: Uint8Array | string,
    }
  }

  export class IdentityEntry extends jspb.Message {
    getKey(): Uint8Array | string;
    getKey_asU8(): Uint8Array;
    getKey_asB64(): string;
    setKey(value: Uint8Array | string): void;

    hasValue(): boolean;
    clearValue(): void;
    getValue(): GetIdentitiesResponse.IdentityValue | undefined;
    setValue(value?: GetIdentitiesResponse.IdentityValue): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): IdentityEntry.AsObject;
    static toObject(includeInstance: boolean, msg: IdentityEntry): IdentityEntry.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: IdentityEntry, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): IdentityEntry;
    static deserializeBinaryFromReader(message: IdentityEntry, reader: jspb.BinaryReader): IdentityEntry;
  }

  export namespace IdentityEntry {
    export type AsObject = {
      key: Uint8Array | string,
      value?: GetIdentitiesResponse.IdentityValue.AsObject,
    }
  }

  export class Identities extends jspb.Message {
    clearIdentityEntriesList(): void;
    getIdentityEntriesList(): Array<GetIdentitiesResponse.IdentityEntry>;
    setIdentityEntriesList(value: Array<GetIdentitiesResponse.IdentityEntry>): void;
    addIdentityEntries(value?: GetIdentitiesResponse.IdentityEntry, index?: number): GetIdentitiesResponse.IdentityEntry;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Identities.AsObject;
    static toObject(includeInstance: boolean, msg: Identities): Identities.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Identities, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Identities;
    static deserializeBinaryFromReader(message: Identities, reader: jspb.BinaryReader): Identities;
  }

  export namespace Identities {
    export type AsObject = {
      identityEntriesList: Array<GetIdentitiesResponse.IdentityEntry.AsObject>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    IDENTITIES = 1,
    PROOF = 2,
  }
}

export class GetIdentityBalanceResponse extends jspb.Message {
  hasBalance(): boolean;
  clearBalance(): void;
  getBalance(): google_protobuf_wrappers_pb.UInt64Value | undefined;
  setBalance(value?: google_protobuf_wrappers_pb.UInt64Value): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentityBalanceResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityBalanceResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityBalanceResponse): GetIdentityBalanceResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityBalanceResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityBalanceResponse;
  static deserializeBinaryFromReader(message: GetIdentityBalanceResponse, reader: jspb.BinaryReader): GetIdentityBalanceResponse;
}

export namespace GetIdentityBalanceResponse {
  export type AsObject = {
    balance?: google_protobuf_wrappers_pb.UInt64Value.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    BALANCE = 1,
    PROOF = 2,
  }
}

export class GetIdentityBalanceAndRevisionResponse extends jspb.Message {
  hasBalanceAndRevision(): boolean;
  clearBalanceAndRevision(): void;
  getBalanceAndRevision(): GetIdentityBalanceAndRevisionResponse.BalanceAndRevision | undefined;
  setBalanceAndRevision(value?: GetIdentityBalanceAndRevisionResponse.BalanceAndRevision): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentityBalanceAndRevisionResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityBalanceAndRevisionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityBalanceAndRevisionResponse): GetIdentityBalanceAndRevisionResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityBalanceAndRevisionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityBalanceAndRevisionResponse;
  static deserializeBinaryFromReader(message: GetIdentityBalanceAndRevisionResponse, reader: jspb.BinaryReader): GetIdentityBalanceAndRevisionResponse;
}

export namespace GetIdentityBalanceAndRevisionResponse {
  export type AsObject = {
    balanceAndRevision?: GetIdentityBalanceAndRevisionResponse.BalanceAndRevision.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class BalanceAndRevision extends jspb.Message {
    hasBalance(): boolean;
    clearBalance(): void;
    getBalance(): google_protobuf_wrappers_pb.UInt64Value | undefined;
    setBalance(value?: google_protobuf_wrappers_pb.UInt64Value): void;

    hasRevision(): boolean;
    clearRevision(): void;
    getRevision(): google_protobuf_wrappers_pb.UInt64Value | undefined;
    setRevision(value?: google_protobuf_wrappers_pb.UInt64Value): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BalanceAndRevision.AsObject;
    static toObject(includeInstance: boolean, msg: BalanceAndRevision): BalanceAndRevision.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BalanceAndRevision, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BalanceAndRevision;
    static deserializeBinaryFromReader(message: BalanceAndRevision, reader: jspb.BinaryReader): BalanceAndRevision;
  }

  export namespace BalanceAndRevision {
    export type AsObject = {
      balance?: google_protobuf_wrappers_pb.UInt64Value.AsObject,
      revision?: google_protobuf_wrappers_pb.UInt64Value.AsObject,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    BALANCE_AND_REVISION = 1,
    PROOF = 2,
  }
}

export class KeyRequestType extends jspb.Message {
  hasAllKeys(): boolean;
  clearAllKeys(): void;
  getAllKeys(): AllKeys | undefined;
  setAllKeys(value?: AllKeys): void;

  hasSpecificKeys(): boolean;
  clearSpecificKeys(): void;
  getSpecificKeys(): SpecificKeys | undefined;
  setSpecificKeys(value?: SpecificKeys): void;

  hasSearchKey(): boolean;
  clearSearchKey(): void;
  getSearchKey(): SearchKey | undefined;
  setSearchKey(value?: SearchKey): void;

  getRequestCase(): KeyRequestType.RequestCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): KeyRequestType.AsObject;
  static toObject(includeInstance: boolean, msg: KeyRequestType): KeyRequestType.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: KeyRequestType, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): KeyRequestType;
  static deserializeBinaryFromReader(message: KeyRequestType, reader: jspb.BinaryReader): KeyRequestType;
}

export namespace KeyRequestType {
  export type AsObject = {
    allKeys?: AllKeys.AsObject,
    specificKeys?: SpecificKeys.AsObject,
    searchKey?: SearchKey.AsObject,
  }

  export enum RequestCase {
    REQUEST_NOT_SET = 0,
    ALL_KEYS = 1,
    SPECIFIC_KEYS = 2,
    SEARCH_KEY = 3,
  }
}

export class AllKeys extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AllKeys.AsObject;
  static toObject(includeInstance: boolean, msg: AllKeys): AllKeys.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AllKeys, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AllKeys;
  static deserializeBinaryFromReader(message: AllKeys, reader: jspb.BinaryReader): AllKeys;
}

export namespace AllKeys {
  export type AsObject = {
  }
}

export class SpecificKeys extends jspb.Message {
  clearKeyIdsList(): void;
  getKeyIdsList(): Array<number>;
  setKeyIdsList(value: Array<number>): void;
  addKeyIds(value: number, index?: number): number;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SpecificKeys.AsObject;
  static toObject(includeInstance: boolean, msg: SpecificKeys): SpecificKeys.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SpecificKeys, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SpecificKeys;
  static deserializeBinaryFromReader(message: SpecificKeys, reader: jspb.BinaryReader): SpecificKeys;
}

export namespace SpecificKeys {
  export type AsObject = {
    keyIdsList: Array<number>,
  }
}

export class SearchKey extends jspb.Message {
  getPurposeMapMap(): jspb.Map<number, SecurityLevelMap>;
  clearPurposeMapMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SearchKey.AsObject;
  static toObject(includeInstance: boolean, msg: SearchKey): SearchKey.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SearchKey, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SearchKey;
  static deserializeBinaryFromReader(message: SearchKey, reader: jspb.BinaryReader): SearchKey;
}

export namespace SearchKey {
  export type AsObject = {
    purposeMapMap: Array<[number, SecurityLevelMap.AsObject]>,
  }
}

export class SecurityLevelMap extends jspb.Message {
  getSecurityLevelMapMap(): jspb.Map<number, SecurityLevelMap.KeyKindRequestTypeMap[keyof SecurityLevelMap.KeyKindRequestTypeMap]>;
  clearSecurityLevelMapMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SecurityLevelMap.AsObject;
  static toObject(includeInstance: boolean, msg: SecurityLevelMap): SecurityLevelMap.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SecurityLevelMap, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SecurityLevelMap;
  static deserializeBinaryFromReader(message: SecurityLevelMap, reader: jspb.BinaryReader): SecurityLevelMap;
}

export namespace SecurityLevelMap {
  export type AsObject = {
    securityLevelMapMap: Array<[number, SecurityLevelMap.KeyKindRequestTypeMap[keyof SecurityLevelMap.KeyKindRequestTypeMap]]>,
  }

  export interface KeyKindRequestTypeMap {
    CURRENT_KEY_OF_KIND_REQUEST: 0;
    ALL_KEYS_OF_KIND_REQUEST: 1;
  }

  export const KeyKindRequestType: KeyKindRequestTypeMap;
}

export class GetIdentityKeysRequest extends jspb.Message {
  getIdentityId(): Uint8Array | string;
  getIdentityId_asU8(): Uint8Array;
  getIdentityId_asB64(): string;
  setIdentityId(value: Uint8Array | string): void;

  hasRequestType(): boolean;
  clearRequestType(): void;
  getRequestType(): KeyRequestType | undefined;
  setRequestType(value?: KeyRequestType): void;

  hasLimit(): boolean;
  clearLimit(): void;
  getLimit(): google_protobuf_wrappers_pb.UInt32Value | undefined;
  setLimit(value?: google_protobuf_wrappers_pb.UInt32Value): void;

  hasOffset(): boolean;
  clearOffset(): void;
  getOffset(): google_protobuf_wrappers_pb.UInt32Value | undefined;
  setOffset(value?: google_protobuf_wrappers_pb.UInt32Value): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityKeysRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityKeysRequest): GetIdentityKeysRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityKeysRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityKeysRequest;
  static deserializeBinaryFromReader(message: GetIdentityKeysRequest, reader: jspb.BinaryReader): GetIdentityKeysRequest;
}

export namespace GetIdentityKeysRequest {
  export type AsObject = {
    identityId: Uint8Array | string,
    requestType?: KeyRequestType.AsObject,
    limit?: google_protobuf_wrappers_pb.UInt32Value.AsObject,
    offset?: google_protobuf_wrappers_pb.UInt32Value.AsObject,
    prove: boolean,
  }
}

export class GetIdentityKeysResponse extends jspb.Message {
  hasKeys(): boolean;
  clearKeys(): void;
  getKeys(): GetIdentityKeysResponse.Keys | undefined;
  setKeys(value?: GetIdentityKeysResponse.Keys): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentityKeysResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityKeysResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityKeysResponse): GetIdentityKeysResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityKeysResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityKeysResponse;
  static deserializeBinaryFromReader(message: GetIdentityKeysResponse, reader: jspb.BinaryReader): GetIdentityKeysResponse;
}

export namespace GetIdentityKeysResponse {
  export type AsObject = {
    keys?: GetIdentityKeysResponse.Keys.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class Keys extends jspb.Message {
    clearKeysBytesList(): void;
    getKeysBytesList(): Array<Uint8Array | string>;
    getKeysBytesList_asU8(): Array<Uint8Array>;
    getKeysBytesList_asB64(): Array<string>;
    setKeysBytesList(value: Array<Uint8Array | string>): void;
    addKeysBytes(value: Uint8Array | string, index?: number): Uint8Array | string;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Keys.AsObject;
    static toObject(includeInstance: boolean, msg: Keys): Keys.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Keys, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Keys;
    static deserializeBinaryFromReader(message: Keys, reader: jspb.BinaryReader): Keys;
  }

  export namespace Keys {
    export type AsObject = {
      keysBytesList: Array<Uint8Array | string>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    KEYS = 1,
    PROOF = 2,
  }
}

export class GetIdentitiesKeysRequest extends jspb.Message {
  clearIdentityIdsList(): void;
  getIdentityIdsList(): Array<Uint8Array | string>;
  getIdentityIdsList_asU8(): Array<Uint8Array>;
  getIdentityIdsList_asB64(): Array<string>;
  setIdentityIdsList(value: Array<Uint8Array | string>): void;
  addIdentityIds(value: Uint8Array | string, index?: number): Uint8Array | string;

  hasRequestType(): boolean;
  clearRequestType(): void;
  getRequestType(): KeyRequestType | undefined;
  setRequestType(value?: KeyRequestType): void;

  hasLimit(): boolean;
  clearLimit(): void;
  getLimit(): google_protobuf_wrappers_pb.UInt32Value | undefined;
  setLimit(value?: google_protobuf_wrappers_pb.UInt32Value): void;

  hasOffset(): boolean;
  clearOffset(): void;
  getOffset(): google_protobuf_wrappers_pb.UInt32Value | undefined;
  setOffset(value?: google_protobuf_wrappers_pb.UInt32Value): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentitiesKeysRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentitiesKeysRequest): GetIdentitiesKeysRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentitiesKeysRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentitiesKeysRequest;
  static deserializeBinaryFromReader(message: GetIdentitiesKeysRequest, reader: jspb.BinaryReader): GetIdentitiesKeysRequest;
}

export namespace GetIdentitiesKeysRequest {
  export type AsObject = {
    identityIdsList: Array<Uint8Array | string>,
    requestType?: KeyRequestType.AsObject,
    limit?: google_protobuf_wrappers_pb.UInt32Value.AsObject,
    offset?: google_protobuf_wrappers_pb.UInt32Value.AsObject,
    prove: boolean,
  }

  export class SecurityLevelMap extends jspb.Message {
    getSecurityLevelMapMap(): jspb.Map<number, GetIdentitiesKeysRequest.SecurityLevelMap.KeyKindRequestTypeMap[keyof GetIdentitiesKeysRequest.SecurityLevelMap.KeyKindRequestTypeMap]>;
    clearSecurityLevelMapMap(): void;
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): SecurityLevelMap.AsObject;
    static toObject(includeInstance: boolean, msg: SecurityLevelMap): SecurityLevelMap.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: SecurityLevelMap, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): SecurityLevelMap;
    static deserializeBinaryFromReader(message: SecurityLevelMap, reader: jspb.BinaryReader): SecurityLevelMap;
  }

  export namespace SecurityLevelMap {
    export type AsObject = {
      securityLevelMapMap: Array<[number, GetIdentitiesKeysRequest.SecurityLevelMap.KeyKindRequestTypeMap[keyof GetIdentitiesKeysRequest.SecurityLevelMap.KeyKindRequestTypeMap]]>,
    }

    export interface KeyKindRequestTypeMap {
      CURRENT_KEY_OF_KIND_REQUEST: 0;
    }

    export const KeyKindRequestType: KeyKindRequestTypeMap;
  }
}

export class GetIdentitiesKeysResponse extends jspb.Message {
  hasPublicKeys(): boolean;
  clearPublicKeys(): void;
  getPublicKeys(): GetIdentitiesKeysResponse.PublicKeyEntries | undefined;
  setPublicKeys(value?: GetIdentitiesKeysResponse.PublicKeyEntries): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentitiesKeysResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentitiesKeysResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentitiesKeysResponse): GetIdentitiesKeysResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentitiesKeysResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentitiesKeysResponse;
  static deserializeBinaryFromReader(message: GetIdentitiesKeysResponse, reader: jspb.BinaryReader): GetIdentitiesKeysResponse;
}

export namespace GetIdentitiesKeysResponse {
  export type AsObject = {
    publicKeys?: GetIdentitiesKeysResponse.PublicKeyEntries.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class PublicKey extends jspb.Message {
    getValue(): Uint8Array | string;
    getValue_asU8(): Uint8Array;
    getValue_asB64(): string;
    setValue(value: Uint8Array | string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): PublicKey.AsObject;
    static toObject(includeInstance: boolean, msg: PublicKey): PublicKey.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: PublicKey, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): PublicKey;
    static deserializeBinaryFromReader(message: PublicKey, reader: jspb.BinaryReader): PublicKey;
  }

  export namespace PublicKey {
    export type AsObject = {
      value: Uint8Array | string,
    }
  }

  export class PublicKeyEntry extends jspb.Message {
    getKey(): Uint8Array | string;
    getKey_asU8(): Uint8Array;
    getKey_asB64(): string;
    setKey(value: Uint8Array | string): void;

    hasValue(): boolean;
    clearValue(): void;
    getValue(): GetIdentitiesKeysResponse.PublicKey | undefined;
    setValue(value?: GetIdentitiesKeysResponse.PublicKey): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): PublicKeyEntry.AsObject;
    static toObject(includeInstance: boolean, msg: PublicKeyEntry): PublicKeyEntry.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: PublicKeyEntry, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): PublicKeyEntry;
    static deserializeBinaryFromReader(message: PublicKeyEntry, reader: jspb.BinaryReader): PublicKeyEntry;
  }

  export namespace PublicKeyEntry {
    export type AsObject = {
      key: Uint8Array | string,
      value?: GetIdentitiesKeysResponse.PublicKey.AsObject,
    }
  }

  export class PublicKeyEntries extends jspb.Message {
    clearPublicKeyEntriesList(): void;
    getPublicKeyEntriesList(): Array<GetIdentitiesKeysResponse.PublicKeyEntry>;
    setPublicKeyEntriesList(value: Array<GetIdentitiesKeysResponse.PublicKeyEntry>): void;
    addPublicKeyEntries(value?: GetIdentitiesKeysResponse.PublicKeyEntry, index?: number): GetIdentitiesKeysResponse.PublicKeyEntry;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): PublicKeyEntries.AsObject;
    static toObject(includeInstance: boolean, msg: PublicKeyEntries): PublicKeyEntries.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: PublicKeyEntries, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): PublicKeyEntries;
    static deserializeBinaryFromReader(message: PublicKeyEntries, reader: jspb.BinaryReader): PublicKeyEntries;
  }

  export namespace PublicKeyEntries {
    export type AsObject = {
      publicKeyEntriesList: Array<GetIdentitiesKeysResponse.PublicKeyEntry.AsObject>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    PUBLIC_KEYS = 1,
    PROOF = 2,
  }
}

export class GetProofsRequest extends jspb.Message {
  clearIdentitiesList(): void;
  getIdentitiesList(): Array<GetProofsRequest.IdentityRequest>;
  setIdentitiesList(value: Array<GetProofsRequest.IdentityRequest>): void;
  addIdentities(value?: GetProofsRequest.IdentityRequest, index?: number): GetProofsRequest.IdentityRequest;

  clearContractsList(): void;
  getContractsList(): Array<GetProofsRequest.ContractRequest>;
  setContractsList(value: Array<GetProofsRequest.ContractRequest>): void;
  addContracts(value?: GetProofsRequest.ContractRequest, index?: number): GetProofsRequest.ContractRequest;

  clearDocumentsList(): void;
  getDocumentsList(): Array<GetProofsRequest.DocumentRequest>;
  setDocumentsList(value: Array<GetProofsRequest.DocumentRequest>): void;
  addDocuments(value?: GetProofsRequest.DocumentRequest, index?: number): GetProofsRequest.DocumentRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetProofsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetProofsRequest): GetProofsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetProofsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetProofsRequest;
  static deserializeBinaryFromReader(message: GetProofsRequest, reader: jspb.BinaryReader): GetProofsRequest;
}

export namespace GetProofsRequest {
  export type AsObject = {
    identitiesList: Array<GetProofsRequest.IdentityRequest.AsObject>,
    contractsList: Array<GetProofsRequest.ContractRequest.AsObject>,
    documentsList: Array<GetProofsRequest.DocumentRequest.AsObject>,
  }

  export class DocumentRequest extends jspb.Message {
    getContractId(): Uint8Array | string;
    getContractId_asU8(): Uint8Array;
    getContractId_asB64(): string;
    setContractId(value: Uint8Array | string): void;

    getDocumentType(): string;
    setDocumentType(value: string): void;

    getDocumentTypeKeepsHistory(): boolean;
    setDocumentTypeKeepsHistory(value: boolean): void;

    getDocumentId(): Uint8Array | string;
    getDocumentId_asU8(): Uint8Array;
    getDocumentId_asB64(): string;
    setDocumentId(value: Uint8Array | string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DocumentRequest.AsObject;
    static toObject(includeInstance: boolean, msg: DocumentRequest): DocumentRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DocumentRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DocumentRequest;
    static deserializeBinaryFromReader(message: DocumentRequest, reader: jspb.BinaryReader): DocumentRequest;
  }

  export namespace DocumentRequest {
    export type AsObject = {
      contractId: Uint8Array | string,
      documentType: string,
      documentTypeKeepsHistory: boolean,
      documentId: Uint8Array | string,
    }
  }

  export class IdentityRequest extends jspb.Message {
    getIdentityId(): Uint8Array | string;
    getIdentityId_asU8(): Uint8Array;
    getIdentityId_asB64(): string;
    setIdentityId(value: Uint8Array | string): void;

    getRequestType(): GetProofsRequest.IdentityRequest.TypeMap[keyof GetProofsRequest.IdentityRequest.TypeMap];
    setRequestType(value: GetProofsRequest.IdentityRequest.TypeMap[keyof GetProofsRequest.IdentityRequest.TypeMap]): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): IdentityRequest.AsObject;
    static toObject(includeInstance: boolean, msg: IdentityRequest): IdentityRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: IdentityRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): IdentityRequest;
    static deserializeBinaryFromReader(message: IdentityRequest, reader: jspb.BinaryReader): IdentityRequest;
  }

  export namespace IdentityRequest {
    export type AsObject = {
      identityId: Uint8Array | string,
      requestType: GetProofsRequest.IdentityRequest.TypeMap[keyof GetProofsRequest.IdentityRequest.TypeMap],
    }

    export interface TypeMap {
      FULL_IDENTITY: 0;
      BALANCE: 1;
      KEYS: 2;
    }

    export const Type: TypeMap;
  }

  export class ContractRequest extends jspb.Message {
    getContractId(): Uint8Array | string;
    getContractId_asU8(): Uint8Array;
    getContractId_asB64(): string;
    setContractId(value: Uint8Array | string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ContractRequest.AsObject;
    static toObject(includeInstance: boolean, msg: ContractRequest): ContractRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ContractRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ContractRequest;
    static deserializeBinaryFromReader(message: ContractRequest, reader: jspb.BinaryReader): ContractRequest;
  }

  export namespace ContractRequest {
    export type AsObject = {
      contractId: Uint8Array | string,
    }
  }
}

export class GetProofsResponse extends jspb.Message {
  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetProofsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetProofsResponse): GetProofsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetProofsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetProofsResponse;
  static deserializeBinaryFromReader(message: GetProofsResponse, reader: jspb.BinaryReader): GetProofsResponse;
}

export namespace GetProofsResponse {
  export type AsObject = {
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }
}

export class GetDataContractRequest extends jspb.Message {
  getId(): Uint8Array | string;
  getId_asU8(): Uint8Array;
  getId_asB64(): string;
  setId(value: Uint8Array | string): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDataContractRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDataContractRequest): GetDataContractRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDataContractRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDataContractRequest;
  static deserializeBinaryFromReader(message: GetDataContractRequest, reader: jspb.BinaryReader): GetDataContractRequest;
}

export namespace GetDataContractRequest {
  export type AsObject = {
    id: Uint8Array | string,
    prove: boolean,
  }
}

export class GetDataContractResponse extends jspb.Message {
  hasDataContract(): boolean;
  clearDataContract(): void;
  getDataContract(): Uint8Array | string;
  getDataContract_asU8(): Uint8Array;
  getDataContract_asB64(): string;
  setDataContract(value: Uint8Array | string): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetDataContractResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDataContractResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDataContractResponse): GetDataContractResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDataContractResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDataContractResponse;
  static deserializeBinaryFromReader(message: GetDataContractResponse, reader: jspb.BinaryReader): GetDataContractResponse;
}

export namespace GetDataContractResponse {
  export type AsObject = {
    dataContract: Uint8Array | string,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    DATA_CONTRACT = 1,
    PROOF = 2,
  }
}

export class GetDataContractsRequest extends jspb.Message {
  clearIdsList(): void;
  getIdsList(): Array<Uint8Array | string>;
  getIdsList_asU8(): Array<Uint8Array>;
  getIdsList_asB64(): Array<string>;
  setIdsList(value: Array<Uint8Array | string>): void;
  addIds(value: Uint8Array | string, index?: number): Uint8Array | string;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDataContractsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDataContractsRequest): GetDataContractsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDataContractsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDataContractsRequest;
  static deserializeBinaryFromReader(message: GetDataContractsRequest, reader: jspb.BinaryReader): GetDataContractsRequest;
}

export namespace GetDataContractsRequest {
  export type AsObject = {
    idsList: Array<Uint8Array | string>,
    prove: boolean,
  }
}

export class GetDataContractsResponse extends jspb.Message {
  hasDataContracts(): boolean;
  clearDataContracts(): void;
  getDataContracts(): GetDataContractsResponse.DataContracts | undefined;
  setDataContracts(value?: GetDataContractsResponse.DataContracts): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetDataContractsResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDataContractsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDataContractsResponse): GetDataContractsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDataContractsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDataContractsResponse;
  static deserializeBinaryFromReader(message: GetDataContractsResponse, reader: jspb.BinaryReader): GetDataContractsResponse;
}

export namespace GetDataContractsResponse {
  export type AsObject = {
    dataContracts?: GetDataContractsResponse.DataContracts.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class DataContractValue extends jspb.Message {
    getValue(): Uint8Array | string;
    getValue_asU8(): Uint8Array;
    getValue_asB64(): string;
    setValue(value: Uint8Array | string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DataContractValue.AsObject;
    static toObject(includeInstance: boolean, msg: DataContractValue): DataContractValue.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DataContractValue, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DataContractValue;
    static deserializeBinaryFromReader(message: DataContractValue, reader: jspb.BinaryReader): DataContractValue;
  }

  export namespace DataContractValue {
    export type AsObject = {
      value: Uint8Array | string,
    }
  }

  export class DataContractEntry extends jspb.Message {
    getKey(): Uint8Array | string;
    getKey_asU8(): Uint8Array;
    getKey_asB64(): string;
    setKey(value: Uint8Array | string): void;

    hasValue(): boolean;
    clearValue(): void;
    getValue(): GetDataContractsResponse.DataContractValue | undefined;
    setValue(value?: GetDataContractsResponse.DataContractValue): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DataContractEntry.AsObject;
    static toObject(includeInstance: boolean, msg: DataContractEntry): DataContractEntry.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DataContractEntry, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DataContractEntry;
    static deserializeBinaryFromReader(message: DataContractEntry, reader: jspb.BinaryReader): DataContractEntry;
  }

  export namespace DataContractEntry {
    export type AsObject = {
      key: Uint8Array | string,
      value?: GetDataContractsResponse.DataContractValue.AsObject,
    }
  }

  export class DataContracts extends jspb.Message {
    clearDataContractEntriesList(): void;
    getDataContractEntriesList(): Array<GetDataContractsResponse.DataContractEntry>;
    setDataContractEntriesList(value: Array<GetDataContractsResponse.DataContractEntry>): void;
    addDataContractEntries(value?: GetDataContractsResponse.DataContractEntry, index?: number): GetDataContractsResponse.DataContractEntry;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DataContracts.AsObject;
    static toObject(includeInstance: boolean, msg: DataContracts): DataContracts.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DataContracts, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DataContracts;
    static deserializeBinaryFromReader(message: DataContracts, reader: jspb.BinaryReader): DataContracts;
  }

  export namespace DataContracts {
    export type AsObject = {
      dataContractEntriesList: Array<GetDataContractsResponse.DataContractEntry.AsObject>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    DATA_CONTRACTS = 1,
    PROOF = 2,
  }
}

export class GetDataContractHistoryRequest extends jspb.Message {
  getId(): Uint8Array | string;
  getId_asU8(): Uint8Array;
  getId_asB64(): string;
  setId(value: Uint8Array | string): void;

  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getStartAtMs(): number;
  setStartAtMs(value: number): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDataContractHistoryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDataContractHistoryRequest): GetDataContractHistoryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDataContractHistoryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDataContractHistoryRequest;
  static deserializeBinaryFromReader(message: GetDataContractHistoryRequest, reader: jspb.BinaryReader): GetDataContractHistoryRequest;
}

export namespace GetDataContractHistoryRequest {
  export type AsObject = {
    id: Uint8Array | string,
    limit: number,
    offset: number,
    startAtMs: number,
    prove: boolean,
  }
}

export class GetDataContractHistoryResponse extends jspb.Message {
  hasDataContractHistory(): boolean;
  clearDataContractHistory(): void;
  getDataContractHistory(): GetDataContractHistoryResponse.DataContractHistory | undefined;
  setDataContractHistory(value?: GetDataContractHistoryResponse.DataContractHistory): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetDataContractHistoryResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDataContractHistoryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDataContractHistoryResponse): GetDataContractHistoryResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDataContractHistoryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDataContractHistoryResponse;
  static deserializeBinaryFromReader(message: GetDataContractHistoryResponse, reader: jspb.BinaryReader): GetDataContractHistoryResponse;
}

export namespace GetDataContractHistoryResponse {
  export type AsObject = {
    dataContractHistory?: GetDataContractHistoryResponse.DataContractHistory.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class DataContractHistoryEntry extends jspb.Message {
    getDate(): number;
    setDate(value: number): void;

    getValue(): Uint8Array | string;
    getValue_asU8(): Uint8Array;
    getValue_asB64(): string;
    setValue(value: Uint8Array | string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DataContractHistoryEntry.AsObject;
    static toObject(includeInstance: boolean, msg: DataContractHistoryEntry): DataContractHistoryEntry.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DataContractHistoryEntry, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DataContractHistoryEntry;
    static deserializeBinaryFromReader(message: DataContractHistoryEntry, reader: jspb.BinaryReader): DataContractHistoryEntry;
  }

  export namespace DataContractHistoryEntry {
    export type AsObject = {
      date: number,
      value: Uint8Array | string,
    }
  }

  export class DataContractHistory extends jspb.Message {
    clearDataContractEntriesList(): void;
    getDataContractEntriesList(): Array<GetDataContractHistoryResponse.DataContractHistoryEntry>;
    setDataContractEntriesList(value: Array<GetDataContractHistoryResponse.DataContractHistoryEntry>): void;
    addDataContractEntries(value?: GetDataContractHistoryResponse.DataContractHistoryEntry, index?: number): GetDataContractHistoryResponse.DataContractHistoryEntry;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DataContractHistory.AsObject;
    static toObject(includeInstance: boolean, msg: DataContractHistory): DataContractHistory.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DataContractHistory, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DataContractHistory;
    static deserializeBinaryFromReader(message: DataContractHistory, reader: jspb.BinaryReader): DataContractHistory;
  }

  export namespace DataContractHistory {
    export type AsObject = {
      dataContractEntriesList: Array<GetDataContractHistoryResponse.DataContractHistoryEntry.AsObject>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    DATA_CONTRACT_HISTORY = 1,
    PROOF = 2,
  }
}

export class GetDocumentsRequest extends jspb.Message {
  getDataContractId(): Uint8Array | string;
  getDataContractId_asU8(): Uint8Array;
  getDataContractId_asB64(): string;
  setDataContractId(value: Uint8Array | string): void;

  getDocumentType(): string;
  setDocumentType(value: string): void;

  getWhere(): Uint8Array | string;
  getWhere_asU8(): Uint8Array;
  getWhere_asB64(): string;
  setWhere(value: Uint8Array | string): void;

  getOrderBy(): Uint8Array | string;
  getOrderBy_asU8(): Uint8Array;
  getOrderBy_asB64(): string;
  setOrderBy(value: Uint8Array | string): void;

  getLimit(): number;
  setLimit(value: number): void;

  hasStartAfter(): boolean;
  clearStartAfter(): void;
  getStartAfter(): Uint8Array | string;
  getStartAfter_asU8(): Uint8Array;
  getStartAfter_asB64(): string;
  setStartAfter(value: Uint8Array | string): void;

  hasStartAt(): boolean;
  clearStartAt(): void;
  getStartAt(): Uint8Array | string;
  getStartAt_asU8(): Uint8Array;
  getStartAt_asB64(): string;
  setStartAt(value: Uint8Array | string): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  getStartCase(): GetDocumentsRequest.StartCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDocumentsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDocumentsRequest): GetDocumentsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDocumentsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDocumentsRequest;
  static deserializeBinaryFromReader(message: GetDocumentsRequest, reader: jspb.BinaryReader): GetDocumentsRequest;
}

export namespace GetDocumentsRequest {
  export type AsObject = {
    dataContractId: Uint8Array | string,
    documentType: string,
    where: Uint8Array | string,
    orderBy: Uint8Array | string,
    limit: number,
    startAfter: Uint8Array | string,
    startAt: Uint8Array | string,
    prove: boolean,
  }

  export enum StartCase {
    START_NOT_SET = 0,
    START_AFTER = 6,
    START_AT = 7,
  }
}

export class GetDocumentsResponse extends jspb.Message {
  hasDocuments(): boolean;
  clearDocuments(): void;
  getDocuments(): GetDocumentsResponse.Documents | undefined;
  setDocuments(value?: GetDocumentsResponse.Documents): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetDocumentsResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDocumentsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDocumentsResponse): GetDocumentsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDocumentsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDocumentsResponse;
  static deserializeBinaryFromReader(message: GetDocumentsResponse, reader: jspb.BinaryReader): GetDocumentsResponse;
}

export namespace GetDocumentsResponse {
  export type AsObject = {
    documents?: GetDocumentsResponse.Documents.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class Documents extends jspb.Message {
    clearDocumentsList(): void;
    getDocumentsList(): Array<Uint8Array | string>;
    getDocumentsList_asU8(): Array<Uint8Array>;
    getDocumentsList_asB64(): Array<string>;
    setDocumentsList(value: Array<Uint8Array | string>): void;
    addDocuments(value: Uint8Array | string, index?: number): Uint8Array | string;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Documents.AsObject;
    static toObject(includeInstance: boolean, msg: Documents): Documents.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Documents, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Documents;
    static deserializeBinaryFromReader(message: Documents, reader: jspb.BinaryReader): Documents;
  }

  export namespace Documents {
    export type AsObject = {
      documentsList: Array<Uint8Array | string>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    DOCUMENTS = 1,
    PROOF = 2,
  }
}

export class GetIdentitiesByPublicKeyHashesRequest extends jspb.Message {
  clearPublicKeyHashesList(): void;
  getPublicKeyHashesList(): Array<Uint8Array | string>;
  getPublicKeyHashesList_asU8(): Array<Uint8Array>;
  getPublicKeyHashesList_asB64(): Array<string>;
  setPublicKeyHashesList(value: Array<Uint8Array | string>): void;
  addPublicKeyHashes(value: Uint8Array | string, index?: number): Uint8Array | string;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentitiesByPublicKeyHashesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentitiesByPublicKeyHashesRequest): GetIdentitiesByPublicKeyHashesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentitiesByPublicKeyHashesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentitiesByPublicKeyHashesRequest;
  static deserializeBinaryFromReader(message: GetIdentitiesByPublicKeyHashesRequest, reader: jspb.BinaryReader): GetIdentitiesByPublicKeyHashesRequest;
}

export namespace GetIdentitiesByPublicKeyHashesRequest {
  export type AsObject = {
    publicKeyHashesList: Array<Uint8Array | string>,
    prove: boolean,
  }
}

export class GetIdentitiesByPublicKeyHashesResponse extends jspb.Message {
  hasIdentities(): boolean;
  clearIdentities(): void;
  getIdentities(): GetIdentitiesByPublicKeyHashesResponse.Identities | undefined;
  setIdentities(value?: GetIdentitiesByPublicKeyHashesResponse.Identities): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentitiesByPublicKeyHashesResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentitiesByPublicKeyHashesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentitiesByPublicKeyHashesResponse): GetIdentitiesByPublicKeyHashesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentitiesByPublicKeyHashesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentitiesByPublicKeyHashesResponse;
  static deserializeBinaryFromReader(message: GetIdentitiesByPublicKeyHashesResponse, reader: jspb.BinaryReader): GetIdentitiesByPublicKeyHashesResponse;
}

export namespace GetIdentitiesByPublicKeyHashesResponse {
  export type AsObject = {
    identities?: GetIdentitiesByPublicKeyHashesResponse.Identities.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export class Identities extends jspb.Message {
    clearIdentitiesList(): void;
    getIdentitiesList(): Array<Uint8Array | string>;
    getIdentitiesList_asU8(): Array<Uint8Array>;
    getIdentitiesList_asB64(): Array<string>;
    setIdentitiesList(value: Array<Uint8Array | string>): void;
    addIdentities(value: Uint8Array | string, index?: number): Uint8Array | string;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Identities.AsObject;
    static toObject(includeInstance: boolean, msg: Identities): Identities.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Identities, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Identities;
    static deserializeBinaryFromReader(message: Identities, reader: jspb.BinaryReader): Identities;
  }

  export namespace Identities {
    export type AsObject = {
      identitiesList: Array<Uint8Array | string>,
    }
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    IDENTITIES = 1,
    PROOF = 2,
  }
}

export class GetIdentityByPublicKeyHashesRequest extends jspb.Message {
  getPublicKeyHash(): Uint8Array | string;
  getPublicKeyHash_asU8(): Uint8Array;
  getPublicKeyHash_asB64(): string;
  setPublicKeyHash(value: Uint8Array | string): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityByPublicKeyHashesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityByPublicKeyHashesRequest): GetIdentityByPublicKeyHashesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityByPublicKeyHashesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityByPublicKeyHashesRequest;
  static deserializeBinaryFromReader(message: GetIdentityByPublicKeyHashesRequest, reader: jspb.BinaryReader): GetIdentityByPublicKeyHashesRequest;
}

export namespace GetIdentityByPublicKeyHashesRequest {
  export type AsObject = {
    publicKeyHash: Uint8Array | string,
    prove: boolean,
  }
}

export class GetIdentityByPublicKeyHashesResponse extends jspb.Message {
  hasIdentity(): boolean;
  clearIdentity(): void;
  getIdentity(): Uint8Array | string;
  getIdentity_asU8(): Uint8Array;
  getIdentity_asB64(): string;
  setIdentity(value: Uint8Array | string): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): GetIdentityByPublicKeyHashesResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIdentityByPublicKeyHashesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIdentityByPublicKeyHashesResponse): GetIdentityByPublicKeyHashesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIdentityByPublicKeyHashesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIdentityByPublicKeyHashesResponse;
  static deserializeBinaryFromReader(message: GetIdentityByPublicKeyHashesResponse, reader: jspb.BinaryReader): GetIdentityByPublicKeyHashesResponse;
}

export namespace GetIdentityByPublicKeyHashesResponse {
  export type AsObject = {
    identity: Uint8Array | string,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    IDENTITY = 1,
    PROOF = 2,
  }
}

export class WaitForStateTransitionResultRequest extends jspb.Message {
  getStateTransitionHash(): Uint8Array | string;
  getStateTransitionHash_asU8(): Uint8Array;
  getStateTransitionHash_asB64(): string;
  setStateTransitionHash(value: Uint8Array | string): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): WaitForStateTransitionResultRequest.AsObject;
  static toObject(includeInstance: boolean, msg: WaitForStateTransitionResultRequest): WaitForStateTransitionResultRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: WaitForStateTransitionResultRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): WaitForStateTransitionResultRequest;
  static deserializeBinaryFromReader(message: WaitForStateTransitionResultRequest, reader: jspb.BinaryReader): WaitForStateTransitionResultRequest;
}

export namespace WaitForStateTransitionResultRequest {
  export type AsObject = {
    stateTransitionHash: Uint8Array | string,
    prove: boolean,
  }
}

export class WaitForStateTransitionResultResponse extends jspb.Message {
  hasError(): boolean;
  clearError(): void;
  getError(): StateTransitionBroadcastError | undefined;
  setError(value?: StateTransitionBroadcastError): void;

  hasProof(): boolean;
  clearProof(): void;
  getProof(): Proof | undefined;
  setProof(value?: Proof): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): ResponseMetadata | undefined;
  setMetadata(value?: ResponseMetadata): void;

  getResultCase(): WaitForStateTransitionResultResponse.ResultCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): WaitForStateTransitionResultResponse.AsObject;
  static toObject(includeInstance: boolean, msg: WaitForStateTransitionResultResponse): WaitForStateTransitionResultResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: WaitForStateTransitionResultResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): WaitForStateTransitionResultResponse;
  static deserializeBinaryFromReader(message: WaitForStateTransitionResultResponse, reader: jspb.BinaryReader): WaitForStateTransitionResultResponse;
}

export namespace WaitForStateTransitionResultResponse {
  export type AsObject = {
    error?: StateTransitionBroadcastError.AsObject,
    proof?: Proof.AsObject,
    metadata?: ResponseMetadata.AsObject,
  }

  export enum ResultCase {
    RESULT_NOT_SET = 0,
    ERROR = 1,
    PROOF = 2,
  }
}

export class ConsensusParamsBlock extends jspb.Message {
  getMaxBytes(): string;
  setMaxBytes(value: string): void;

  getMaxGas(): string;
  setMaxGas(value: string): void;

  getTimeIotaMs(): string;
  setTimeIotaMs(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConsensusParamsBlock.AsObject;
  static toObject(includeInstance: boolean, msg: ConsensusParamsBlock): ConsensusParamsBlock.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ConsensusParamsBlock, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConsensusParamsBlock;
  static deserializeBinaryFromReader(message: ConsensusParamsBlock, reader: jspb.BinaryReader): ConsensusParamsBlock;
}

export namespace ConsensusParamsBlock {
  export type AsObject = {
    maxBytes: string,
    maxGas: string,
    timeIotaMs: string,
  }
}

export class ConsensusParamsEvidence extends jspb.Message {
  getMaxAgeNumBlocks(): string;
  setMaxAgeNumBlocks(value: string): void;

  getMaxAgeDuration(): string;
  setMaxAgeDuration(value: string): void;

  getMaxBytes(): string;
  setMaxBytes(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConsensusParamsEvidence.AsObject;
  static toObject(includeInstance: boolean, msg: ConsensusParamsEvidence): ConsensusParamsEvidence.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ConsensusParamsEvidence, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConsensusParamsEvidence;
  static deserializeBinaryFromReader(message: ConsensusParamsEvidence, reader: jspb.BinaryReader): ConsensusParamsEvidence;
}

export namespace ConsensusParamsEvidence {
  export type AsObject = {
    maxAgeNumBlocks: string,
    maxAgeDuration: string,
    maxBytes: string,
  }
}

export class GetConsensusParamsRequest extends jspb.Message {
  getHeight(): number;
  setHeight(value: number): void;

  getProve(): boolean;
  setProve(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetConsensusParamsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetConsensusParamsRequest): GetConsensusParamsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetConsensusParamsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetConsensusParamsRequest;
  static deserializeBinaryFromReader(message: GetConsensusParamsRequest, reader: jspb.BinaryReader): GetConsensusParamsRequest;
}

export namespace GetConsensusParamsRequest {
  export type AsObject = {
    height: number,
    prove: boolean,
  }
}

export class GetConsensusParamsResponse extends jspb.Message {
  hasBlock(): boolean;
  clearBlock(): void;
  getBlock(): ConsensusParamsBlock | undefined;
  setBlock(value?: ConsensusParamsBlock): void;

  hasEvidence(): boolean;
  clearEvidence(): void;
  getEvidence(): ConsensusParamsEvidence | undefined;
  setEvidence(value?: ConsensusParamsEvidence): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetConsensusParamsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetConsensusParamsResponse): GetConsensusParamsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetConsensusParamsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetConsensusParamsResponse;
  static deserializeBinaryFromReader(message: GetConsensusParamsResponse, reader: jspb.BinaryReader): GetConsensusParamsResponse;
}

export namespace GetConsensusParamsResponse {
  export type AsObject = {
    block?: ConsensusParamsBlock.AsObject,
    evidence?: ConsensusParamsEvidence.AsObject,
  }
}

