const Certificate = require('./Certificate');
const requestApi = require('./requestApi');

/**
 * Get ZeroSSL certificate
 *
 * @typedef {getCertificate}
 * @param {string} apiKey
 * @param {string} id
 * @return {Promise<Certificate>}
 */
async function getCertificate(apiKey, id) {
  const url = `https://api.zerossl.com/certificates/${id}?access_key=${apiKey}`;

  const requestOptions = {
    method: 'GET',
    headers: { },
  };

  const data = await requestApi(url, requestOptions);

  return new Certificate(data);
}

module.exports = getCertificate;
