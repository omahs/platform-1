const {
  tendermint: {
    abci: {
      ResponseQuery,
    },
  },
} = require('@dashevo/abci/types');

const {
  v0: {
    GetDataContractResponse,
  },
} = require('@dashevo/dapi-grpc');

const NotFoundAbciError = require('../../errors/NotFoundAbciError');
const InvalidArgumentAbciError = require('../../errors/InvalidArgumentAbciError');

/**
 *
 * @param {DataContractStoreRepository} dataContractRepository
 * @param {createQueryResponse} createQueryResponse
 * @param {WebAssembly.Instance} dppWasm
 * @return {dataContractQueryHandler}
 */
function dataContractQueryHandlerFactory(
  dataContractRepository,
  createQueryResponse,
  dppWasm,
) {
  /**
   * @typedef dataContractQueryHandler
   * @param {Object} params
   * @param {Object} data
   * @param {Buffer} data.id
   * @param {RequestQuery} request
   * @return {Promise<ResponseQuery>}
   */
  async function dataContractQueryHandler(params, { id }, request) {
    let contractIdIdentifier;
    try {
      contractIdIdentifier = new dppWasm.Identifier(id);
    } catch (e) {
      if (e instanceof dppWasm.IdentifierError) {
        throw new InvalidArgumentAbciError('id must be a valid identifier (32 bytes long)');
      }

      throw e;
    }

    const response = createQueryResponse(GetDataContractResponse, request.prove);

    if (request.prove) {
      const proof = await dataContractRepository.prove(contractIdIdentifier);

      response.getProof().setMerkleProof(proof.getValue());
    } else {
      const dataContract = await dataContractRepository.fetch(contractIdIdentifier);
      if (dataContract.isNull()) {
        throw new NotFoundAbciError('Data Contract not found');
      }

      response.setDataContract(dataContract.getValue().toBuffer());
    }

    return new ResponseQuery({
      value: response.serializeBinary(),
    });
  }

  return dataContractQueryHandler;
}

module.exports = dataContractQueryHandlerFactory;
