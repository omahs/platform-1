const decodeProtocolEntityFactory = require('@dashevo/dpp/lib/decodeProtocolEntityFactory');
const DataContract = require('@dashevo/dpp/lib/dataContract/DataContract');

const decodeProtocolEntity = decodeProtocolEntityFactory();

class DataContractStoreRepository {
  /**
   *
   * @param {GroveDBStore} groveDBStore
   */
  constructor(groveDBStore) {
    this.storage = groveDBStore;
  }

  /**
   * Store Data Contract into database
   *
   * @param {DataContract} dataContract
   * @param {GroveDBTransaction} [transaction]
   * @return {Promise<DataContractStoreRepository>}
   */
  async store(dataContract, transaction = undefined) {
    // Create contract tree
    await this.storage.createTree(
      DataContractStoreRepository.TREE_PATH,
      dataContract.getId().toBuffer(),
      { transaction },
    );

    const contractTreePath = DataContractStoreRepository.TREE_PATH
      .concat([dataContract.getId().toBuffer()]);

    // Store contract
    await this.storage.put(
      contractTreePath,
      DataContractStoreRepository.DATA_CONTRACT_REFERENCE_KEY,
      dataContract.toBuffer(), // TODO: we should use references
      { transaction },
    );

    /**
     * Create document type trees
     */
    const promises = Object.entries(dataContract.getDocuments())
      .map(async ([documentType, documentDefinition]) => {
        // Create document type tree
        await this.storage.createTree(
          contractTreePath,
          Buffer.from(documentType),
          { transaction, skipIfExists: true },
        );

        const documentTypeTreePath = contractTreePath.concat([Buffer.from(documentType)]);

        // Create $id tree
        await this.storage.createTree(
          documentTypeTreePath,
          DataContractStoreRepository.ID_TREE_KEY,
          { transaction, skipIfExists: true },
        );

        // Create first indexed property trees
        const firstIndexedProperties = (documentDefinition.indices || []).map((indexDefinition) => (
          Object.keys(indexDefinition.properties[0])[0]
        ));

        const uniqueFirstIndexedProperties = [...new Set(firstIndexedProperties)];

        await Promise.all(uniqueFirstIndexedProperties.map(async (indexedProperty) => {
          // Create tree for indexed property
          await this.storage.createTree(
            documentTypeTreePath,
            Buffer.from(indexedProperty),
            { transaction, skipIfExists: true },
          );

          // Create tree for ID references
          await this.storage.createTree(
            documentTypeTreePath.concat([Buffer.from(indexedProperty)]),
            DataContractStoreRepository.ID_TREE_KEY,
            { transaction, skipIfExists: true },
          );
        }));
      });

    await Promise.all(promises);

    return this;
  }

  /**
   * Fetch Data Contract by ID from database
   *
   * @param {Identifier} id
   * @param {GroveDBTransaction} [transaction]
   * @return {Promise<null|DataContract>}
   */
  async fetch(id, transaction = undefined) {
    const encodedDataContract = await this.storage.get(
      DataContractStoreRepository.TREE_PATH.concat([id.toBuffer()]),
      Buffer.from(0),
      { transaction },
    );

    if (!encodedDataContract) {
      return null;
    }

    const [, rawDataContract] = decodeProtocolEntity(encodedDataContract);

    return new DataContract(rawDataContract);
  }

  /**
   * @return {Promise<DataContractStoreRepository>}
   */
  async createTree() {
    await this.storage.createTree([], DataContractStoreRepository.TREE_PATH[0]);

    return this;
  }
}

DataContractStoreRepository.TREE_PATH = [Buffer.from('contracts')];
DataContractStoreRepository.DATA_CONTRACT_REFERENCE_KEY = Buffer.from(0);
DataContractStoreRepository.ID_TREE_KEY = Buffer.from(0);

module.exports = DataContractStoreRepository;
