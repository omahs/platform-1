const { Listr } = require('listr2');

const { Flags } = require('@oclif/core');

const ConfigBaseCommand = require('../oclif/command/ConfigBaseCommand');

const MuteOneLineError = require('../oclif/errors/MuteOneLineError');

class ResetCommand extends ConfigBaseCommand {
  /**
   * @param {Object} args
   * @param {Object} flags
   * @param {isSystemConfig} isSystemConfig
   * @param {Config} config
   * @param {resetNodeTask} resetNodeTask
   *
   * @return {Promise<void>}
   */
  async runWithDependencies(
    args,
    {
      verbose: isVerbose,
      hard: isHardReset,
      force: isForce,
      platform: isPlatformOnlyReset,
    },
    isSystemConfig,
    config,
    resetNodeTask,
  ) {
    const tasks = new Listr([
      {
        title: `Reset ${config.getName()} node`,
        task: () => resetNodeTask(config),
      },
    ],
    {
      renderer: isVerbose ? 'verbose' : 'default',
      rendererOptions: {
        showTimer: isVerbose,
        clearOutput: false,
        collapse: false,
        showSubtasks: true,
      },
    });

    try {
      await tasks.run({
        isHardReset,
        isPlatformOnlyReset,
        isForce,
        isVerbose,
      });
    } catch (e) {
      throw new MuteOneLineError(e);
    }
  }
}

ResetCommand.description = 'Reset node data';

ResetCommand.flags = {
  ...ConfigBaseCommand.flags,
  hard: Flags.boolean({ char: 'h', description: 'reset config as well as data', default: false }),
  force: Flags.boolean({ char: 'f', description: 'skip running services check', default: false }),
  platform: Flags.boolean({ char: 'p', description: 'reset platform services and data only', default: false }),
  verbose: Flags.boolean({ char: 'v', description: 'use verbose mode for output', default: false }),
};

module.exports = ResetCommand;
