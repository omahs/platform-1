const chalk = require('chalk');

const { NODE_TYPE_MASTERNODE } = require('../../../../constants');
const getBLSPublicKeyFromPrivateKeyHex = require('../../../../core/getBLSPublicKeyFromPrivateKeyHex');

async function getConfigurationOutputFromContext(ctx) {
  let output = '';
  if (ctx.nodeType === NODE_TYPE_MASTERNODE) {
    const publicKeyHex = await getBLSPublicKeyFromPrivateKeyHex(
      ctx.config.get('core.masternode.operator.privateKey'),
    );

    output += chalk`Operator Public key: ${publicKeyHex}
                    Operator Private key: ******** - view with {bold.cyanBright dashmate config get core.masternode.operator.privateKey}\n\n`;
  }

  if (ctx.isHP) {
    output += chalk`Platform Node ID: ${ctx.config.get('platform.drive.tenderdash.node.id')}
                    Platform Node key: ******** - view with {bold.cyanBright dashmate config get platform.drive.tenderdash.node.key}\n\n`;
  }

  output += `External IP: ${ctx.config.get('externalIp') || 'auto'}
             Core P2P port: ${ctx.config.get('core.p2p.port')}`;

  if (ctx.isHP) {
    output += `\n\nPlatform P2P port: ${ctx.config.get('platform.drive.tenderdash.p2p.port')}
                   Platform HTTP port: ${ctx.config.get('platform.dapi.envoy.http.port')}`;
  }

  return output;
}

module.exports = getConfigurationOutputFromContext;
