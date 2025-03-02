import { expect } from 'chai';
import { latestVersion as latestProtocolVersion } from '@dashevo/dpp/lib/version/protocolVersion';
import { Platform } from './index';
import 'mocha';
import Client from '../Client';

describe('Dash - Platform', () => {
  it('should provide expected class', () => {
    expect(Platform.name).to.be.equal('Platform');
    expect(Platform.constructor.name).to.be.equal('Function');
  });

  it('should set protocol version for DPP though options', async () => {
    const platform = new Platform({
      client: new Client(),
      network: 'testnet',
      driveProtocolVersion: 42,
    });

    await platform.initialize();
    expect(platform.dpp.getProtocolVersion()).to.equal(42);
  });

  it('should set protocol version for DPP using mapping', async () => {
    const platform = new Platform({
      client: new Client(),
      network: 'testnet',
    });

    // @ts-ignore
    const testnetProtocolVersion = Platform.networkToProtocolVersion.get('testnet');

    await platform.initialize();
    expect(platform.dpp.getProtocolVersion()).to.equal(testnetProtocolVersion);
  });

  it('should set protocol version for DPP using latest version', async () => {
    const platform = new Platform({
      client: new Client(),
      network: 'unknown',
    });

    await platform.initialize();
    expect(platform.dpp.getProtocolVersion()).to.equal(latestProtocolVersion);
  });
});
