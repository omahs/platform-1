const os = require('os');
const path = require('path');
const { Observable } = require('rxjs');

const isWsl = require('is-wsl');

const dockerCompose = require('@dashevo/docker-compose');

const hasbin = require('hasbin');
const semver = require('semver');
const yaml = require('js-yaml');
const fs = require('fs');

const DockerComposeError = require('./errors/DockerComposeError');
const ServiceAlreadyRunningError = require('./errors/ServiceAlreadyRunningError');
const ServiceIsNotRunningError = require('./errors/ServiceIsNotRunningError');
const ContainerIsNotPresentError = require('./errors/ContainerIsNotPresentError');

const { HOME_DIR_PATH, PACKAGE_ROOT_DIR } = require('../constants');

class DockerCompose {
  /**
   * @param {Docker} docker
   * @param {StartedContainers} startedContainers
   */
  constructor(docker, startedContainers) {
    this.docker = docker;
    this.startedContainers = startedContainers;
    this.isDockerSetupVerified = false;
  }

  /**
   * Run service
   *
   * @param {Object} envs
   * @param {string} serviceName
   * @param {array} [command]
   * @param {array} [options]
   * @return {Promise<Container>}
   */
  async runService(envs, serviceName, command = [], options = []) {
    await this.throwErrorIfNotInstalled();

    if (await this.isServiceRunning(envs, serviceName)) {
      throw new ServiceAlreadyRunningError(serviceName);
    }

    let containerName;

    try {
      ({ out: containerName } = await dockerCompose.run(
        serviceName,
        command,
        {
          ...this.getOptions(envs),
          commandOptions: options,
        },
      ));
    } catch (e) {
      throw new DockerComposeError(e);
    }

    containerName = containerName.trim().split(/\r?\n/).pop();

    this.startedContainers.addContainer(containerName);

    return this.docker.getContainer(containerName);
  }

  /**
   * Checks if node is running by checking whether first container
   * from the targeted node is in `running` state
   *
   * @param {Object} envs
   * @return {Promise<boolean>}
   */
  async isNodeRunning(envs) {
    await this.throwErrorIfNotInstalled();

    const targetedComposeFiles = envs.COMPOSE_FILE.split(':');

    const services = targetedComposeFiles
      .map((composeFile) => yaml.load(fs.readFileSync(path.join(PACKAGE_ROOT_DIR, composeFile), 'utf8')))
      .map((composeFile) => Object.keys(composeFile.services))
      .flat()
      .filter((value, index, array) => array.indexOf(value) === index);

    const serviceContainers = await this.getContainersList(envs, {
      filterServiceNames: services,
      formatJson: true,
    });

    for (const { State: state } of serviceContainers) {
      if (state === 'running') {
        return true;
      }
    }

    return false;
  }

  /**
   * Checks if service is running
   *
   * @param {Object} envs
   * @param {string} serviceName filter by service name
   * @return {Promise<boolean>}
   */
  async isServiceRunning(envs, serviceName) {
    await this.throwErrorIfNotInstalled();

    const [container] = await this.getContainersList(envs, {
      filterServiceNames: serviceName,
      formatJson: true,
    });

    return container?.State === 'running';
  }

  /**
   * Up docker compose
   *
   * @param {Object} envs
   * @return {Promise<void>}
   */
  async up(envs) {
    await this.throwErrorIfNotInstalled();

    try {
      await dockerCompose.upAll({
        ...this.getOptions(envs),
        commandOptions: ['--no-build'],
      });
    } catch (e) {
      throw new DockerComposeError(e);
    }
  }

  /**
   * Build docker compose images
   *
   * @param {Object} envs
   * @param {string} [serviceName]
   * @param {Array} [options]
   * @return {Observable<{string}>}
   */
  // eslint-disable-next-line no-unused-vars
  async build(envs, serviceName = undefined, options = []) {
    try {
      return new Observable(async (observer) => {
        await this.throwErrorIfNotInstalled();

        const callback = (e) => {
          observer.next(e.toString());
        };

        if (serviceName) {
          await dockerCompose.buildOne(serviceName, {
            ...this.getOptions(envs),
            callback,
            commandOptions: options,
          });
        } else {
          await dockerCompose.buildAll({
            ...this.getOptions(envs),
            callback,
            commandOptions: options,
          });
        }

        observer.complete();
      });
    } catch (e) {
      throw new DockerComposeError(e);
    }
  }

  /**
   * Stop all docker compose containers
   *
   * @param {Object} envs
   * @return {Promise<void>}
   */
  async stop(envs) {
    await this.throwErrorIfNotInstalled();

    try {
      await dockerCompose.stop(this.getOptions(envs));
    } catch (e) {
      throw new DockerComposeError(e);
    }
  }

  /**
   * Inspect service
   *
   * @param {Object} envs
   * @param {string} serviceName
   * @return {Promise<object>}
   */
  async inspectService(envs, serviceName) {
    await this.throwErrorIfNotInstalled();

    const containerIds = await this.getContainersList(envs, {
      filterServiceNames:
      serviceName,
      quiet: true,
    });

    if (containerIds.length === 0) {
      throw new ContainerIsNotPresentError(serviceName);
    }

    const container = this.docker.getContainer(containerIds[0]);

    return container.inspect();
  }

  /**
   * Execute command
   *
   * @param {Object} envs
   * @param {string} serviceName
   * @param {string} command
   * @param {string[]} [commandOptions]
   * @return {Promise<object>}
   */
  async execCommand(envs, serviceName, command, commandOptions = []) {
    await this.throwErrorIfNotInstalled();

    if (!(await this.isServiceRunning(envs, serviceName))) {
      throw new ServiceIsNotRunningError(envs.CONFIG_NAME, serviceName);
    }

    let commandOutput;

    const options = {
      ...this.getOptions(envs),
      commandOptions,
    };

    try {
      commandOutput = await dockerCompose.exec(
        serviceName,
        command,
        options,
      );
    } catch (e) {
      throw new DockerComposeError(e);
    }

    return commandOutput;
  }

  /**
   * Get list of Docker containers
   *
   * @param {Object} envs
   * @param {Object} [options={}] optional
   * @param {string|string[]} [options.filterServiceNames=false] - Filter by service name
   * @param {boolean} [options.returnServiceNames] - Return only service names
   * @param {boolean} [options.quiet=false] - Return only container ids
   * @param {boolean} [options.formatJson=false] - Return as json with details
   * @return {Promise<string[]|object[]>}
   */
  async getContainersList(
    envs,
    {
      filterServiceNames = undefined,
      returnServiceNames = false,
      quiet = false,
      formatJson = false,
    } = {},
  ) {
    let psOutput;
    const commandOptions = [];

    if (returnServiceNames) {
      commandOptions.push('--services');
    }

    if (quiet) {
      commandOptions.push('--quiet');
    }

    if (formatJson) {
      commandOptions.push('--format', 'json');
    }

    commandOptions.push(filterServiceNames);

    try {
      ({ out: psOutput } = await dockerCompose.ps({
        ...this.getOptions(envs),
        commandOptions,
      }));
    } catch (e) {
      if (e.err && e.err.startsWith('no such service:')) {
        return [];
      }

      throw new DockerComposeError(e);
    }

    const containerList = psOutput
      .trim()
      .split(/\r?\n/)
      .filter(Boolean);

    if (containerList.length > 0 && formatJson) {
      return JSON.parse(containerList[0]);
    }

    return containerList;
  }

  /**
   * Get list of Docker volumes
   * @param {Object} envs
   * @return {Promise<string[]>}
   */
  async getVolumeNames(envs) {
    let volumeOutput;
    try {
      ({ out: volumeOutput } = await dockerCompose.configVolumes({
        ...this.getOptions(envs),
      }));
    } catch (e) {
      throw new DockerComposeError(e);
    }

    return volumeOutput
      .trim()
      .split(/\r?\n/);
  }

  /**
   * Down docker compose
   *
   * @param {Object} envs
   * @return {Promise<void>}
   */
  async down(envs) {
    await this.throwErrorIfNotInstalled();

    try {
      await dockerCompose.down({
        ...this.getOptions(envs),
        commandOptions: ['-v', '--remove-orphans'],
      });
    } catch (e) {
      throw new DockerComposeError(e);
    }
  }

  /**
   * Remove docker compose
   *
   * @param {Object} envs
   * @param {string[]} [serviceNames]
   * @return {Promise<void>}
   */
  async rm(envs, serviceNames) {
    await this.throwErrorIfNotInstalled();

    try {
      await dockerCompose.rm({
        ...this.getOptions(envs),
        commandOptions: ['--stop'],
      }, ...serviceNames);
    } catch (e) {
      throw new DockerComposeError(e);
    }
  }

  /**
   * Pull docker compose
   *
   * @param {Object} envs
   * @return {Promise<void>}
   */
  async pull(envs) {
    await this.throwErrorIfNotInstalled();

    try {
      await dockerCompose.pullAll({
        ...this.getOptions(envs),
        commandOptions: ['-q'],
      });
    } catch (e) {
      throw new DockerComposeError(e);
    }
  }

  /**
   * @return {Promise<void>}
   */
  async throwErrorIfNotInstalled() {
    if (this.isDockerSetupVerified) {
      return;
    }

    this.isDockerSetupVerified = true;

    const dockerComposeInstallLink = 'https://docs.docker.com/compose/install/';
    const dockerInstallLink = 'https://docs.docker.com/engine/install/';
    const dockerPostInstallLinuxLink = 'https://docs.docker.com/engine/install/linux-postinstall/';
    const dockerContextLink = 'https://docs.docker.com/engine/context/working-with-contexts/';

    // Check docker
    if (!hasbin.sync('docker')) {
      throw new Error(`Docker is not installed. Please follow instructions ${dockerInstallLink}`);
    }

    let dockerVersion;
    try {
      dockerVersion = await new Promise((resolve, reject) => {
        this.docker.version((err, data) => {
          if (err) {
            return reject(err);
          }

          return resolve(data.Version);
        });
      });
    } catch (e) {
      throw new Error(`Can't connect to Docker Engine: ${e.message}.\n\nPossible reasons:\n1. Docker is not started\n2. Permission issues ${dockerPostInstallLinuxLink}\n3. Wrong context ${dockerContextLink}`);
    }

    if (semver.lt(dockerVersion.trim(), DockerCompose.DOCKER_MIN_VERSION)) {
      throw new Error(`Update Docker to version ${DockerCompose.DOCKER_MIN_VERSION} or higher. Please follow instructions ${dockerInstallLink}`);
    }

    let version;

    // Check docker compose
    try {
      ({ out: version } = await dockerCompose.version());
    } catch (e) {
      throw new Error(`Docker Compose V2 is not available in your system. Please follow instructions ${dockerComposeInstallLink}`);
    }

    if (semver.lt(version.trim(), DockerCompose.DOCKER_COMPOSE_MIN_VERSION)) {
      throw new Error(`Update Docker Compose to version ${DockerCompose.DOCKER_COMPOSE_MIN_VERSION} or higher. Please follow instructions ${dockerComposeInstallLink}`);
    }
  }

  /**
   * @private
   * @param {Object} envs
   * @return {{cwd: string, env: Object}}
   */
  getOptions(envs) {
    const { uid, gid } = os.userInfo();

    const env = {
      ...process.env,
      ...envs,
      DASHMATE_HOME_DIR: HOME_DIR_PATH,
      LOCAL_UID: uid,
      LOCAL_GID: gid,
    };

    if (isWsl) {
      // Solving issue under WSL when after restart container volume is not being mounted properly
      // https://github.com/docker/for-win/issues/4812
      // Following fix forces container recreation
      env.WSL2_FIX = (new Date()).getTime();
    }

    return {
      cwd: path.join(__dirname, '..', '..'),
      env,
    };
  }

  /**
   * Resolve container internal IP
   *
   * @param {Object} envs
   * @param {string} serviceName
   * @return {Promise<string>}
   */
  async getContainerIp(envs, serviceName) {
    const containerInfo = await this.inspectService(envs, serviceName);

    const [firstNetwork] = Object.keys(containerInfo.NetworkSettings.Networks);
    const { IPAddress: containerIP } = containerInfo.NetworkSettings.Networks[firstNetwork];

    return containerIP;
  }
}

DockerCompose.DOCKER_COMPOSE_MIN_VERSION = '2.0.0';
DockerCompose.DOCKER_MIN_VERSION = '20.10.0';

module.exports = DockerCompose;
