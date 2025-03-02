const crypto = require('crypto');
const BlsSignatures = require('@dashevo/bls');

/**
 * Generate BLS keys
 *
 * @typedef {generateBlsKeys}
 * @return {Promise<{privateKey: *, address: *}>}
 */
async function generateBlsKeys() {
  const blsSignatures = await BlsSignatures();
  const { BasicSchemeMPL } = blsSignatures;

  const randomBytes = new Uint8Array(crypto.randomBytes(256));
  const operatorPrivateKey = BasicSchemeMPL.keyGen(randomBytes);
  const operatorPublicKey = operatorPrivateKey.getG1();

  const operatorPrivateKeyHex = Buffer.from(operatorPrivateKey.serialize()).toString('hex');
  const operatorPublicKeyHex = Buffer.from(operatorPublicKey.serialize()).toString('hex');

  operatorPrivateKey.delete();
  operatorPublicKey.delete();

  return {
    publicKey: operatorPublicKeyHex,
    privateKey: operatorPrivateKeyHex,
  };
}

module.exports = generateBlsKeys;
