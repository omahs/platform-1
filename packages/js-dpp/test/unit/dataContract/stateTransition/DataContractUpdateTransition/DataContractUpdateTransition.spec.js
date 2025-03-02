const varint = require('varint');
const getDataContractFixture = require('../../../../../lib/test/fixtures/getDataContractFixture');
const stateTransitionTypes = require('../../../../../lib/stateTransition/stateTransitionTypes');

const Identifier = require('../../../../../lib/identifier/Identifier');
const protocolVersion = require('../../../../../lib/version/protocolVersion');
const DataContractUpdateTransition = require('../../../../../lib/dataContract/stateTransition/DataContractUpdateTransition/DataContractUpdateTransition');
const hash = require('../../../../../lib/util/hash');
const serializer = require('../../../../../lib/util/serializer');

describe('DataContractUpdateTransition', () => {
  let stateTransition;
  let dataContract;
  let hashMock;
  let encodeMock;

  beforeEach(function beforeEach() {
    dataContract = getDataContractFixture();

    encodeMock = this.sinonSandbox.stub(serializer, 'encode');
    hashMock = this.sinonSandbox.stub(hash, 'hash');

    stateTransition = new DataContractUpdateTransition({
      protocolVersion: protocolVersion.latestVersion,
      dataContract: dataContract.toObject(),
    });
  });

  afterEach(() => {
    encodeMock.restore();
    hashMock.restore();
  });

  describe('#getProtocolVersion', () => {
    it('should return the current protocol version', () => {
      const result = stateTransition.getProtocolVersion();

      expect(result).to.equal(protocolVersion.latestVersion);
    });
  });

  describe('#getType', () => {
    it('should return State Transition type', () => {
      const result = stateTransition.getType();

      expect(result).to.equal(stateTransitionTypes.DATA_CONTRACT_UPDATE);
    });
  });

  describe('#getDataContract', () => {
    it('should return Data Contract', () => {
      const result = stateTransition.getDataContract();

      expect(result.toObject()).to.deep.equal(dataContract.toObject());
    });
  });

  describe('#toJSON', () => {
    it('should return State Transition as plain JS object', () => {
      expect(stateTransition.toJSON()).to.deep.equal({
        protocolVersion: protocolVersion.latestVersion,
        type: stateTransitionTypes.DATA_CONTRACT_UPDATE,
        dataContract: dataContract.toJSON(),
        signaturePublicKeyId: undefined,
        signature: undefined,
      });
    });
  });

  describe('#toBuffer', () => {
    it('should return serialized State Transition', () => {
      const serializedStateTransition = Buffer.from('123');

      encodeMock.returns(serializedStateTransition);

      const protocolVersionBytes = Buffer.from(varint.encode(stateTransition.protocolVersion));

      const result = stateTransition.toBuffer();

      expect(result).to.deep.equal(
        Buffer.concat([protocolVersionBytes, serializedStateTransition]),
      );

      const dataToEncode = stateTransition.toObject();
      delete dataToEncode.protocolVersion;

      expect(encodeMock.getCall(0).args).to.have.deep.members([
        dataToEncode,
      ]);
    });
  });

  describe('#hash', () => {
    it('should return State Transition hash as hex', () => {
      const serializedDocument = Buffer.from('123');
      const hashedDocument = '456';

      encodeMock.returns(serializedDocument);
      hashMock.returns(hashedDocument);

      const result = stateTransition.hash();

      expect(result).to.equal(hashedDocument);

      const dataToEncode = stateTransition.toObject();
      delete dataToEncode.protocolVersion;

      expect(encodeMock.getCall(0).args).to.have.deep.members([
        dataToEncode,
      ]);

      const protocolVersionBytes = Buffer.from(varint.encode(stateTransition.protocolVersion));

      expect(hashMock).to.have.been.calledOnceWith(
        Buffer.concat([protocolVersionBytes, serializedDocument]),
      );
    });
  });

  describe('#getOwnerId', () => {
    it('should return owner id', async () => {
      const result = stateTransition.getOwnerId();

      expect(result).to.equal(stateTransition.getDataContract().getOwnerId());
    });
  });

  describe('#getModifiedDataIds', () => {
    it('should return ids of affected data contracts', () => {
      const result = stateTransition.getModifiedDataIds();

      expect(result.length).to.be.equal(1);
      const contractId = result[0];

      expect(contractId).to.be.an.instanceOf(Identifier);
      expect(contractId).to.be.deep.equal(dataContract.getId());
    });
  });

  describe('#isDataContractStateTransition', () => {
    it('should return true', () => {
      expect(stateTransition.isDataContractStateTransition()).to.be.true();
    });
  });

  describe('#isDocumentStateTransition', () => {
    it('should return false', () => {
      expect(stateTransition.isDocumentStateTransition()).to.be.false();
    });
  });

  describe('#isIdentityStateTransition', () => {
    it('should return false', () => {
      expect(stateTransition.isIdentityStateTransition()).to.be.false();
    });
  });
});
