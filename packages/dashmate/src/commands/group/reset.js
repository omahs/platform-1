const { Listr } = require('listr2');

const { Flags } = require('@oclif/core');

const GroupBaseCommand = require('../../oclif/command/GroupBaseCommand');
const MuteOneLineError = require('../../oclif/errors/MuteOneLineError');
const { PRESET_LOCAL } = require('../../constants');

class GroupResetCommand extends GroupBaseCommand {
  /**
   * @param {Object} args
   * @param {Object} flags
   * @param {isSystemConfig} isSystemConfig
   * @param {resetNodeTask} resetNodeTask
   * @param {Config[]} configGroup
   * @param {configureCoreTask} configureCoreTask
   * @param {configureTenderdashTask} configureTenderdashTask
   * @param {generateToAddressTask} generateToAddressTask
   * @param {ConfigFile} configFile
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
    resetNodeTask,
    configGroup,
    configureCoreTask,
    configureTenderdashTask,
    generateToAddressTask,
    configFile,
  ) {
    const groupName = configGroup[0].get('group');

    const tasks = new Listr(
      [
        {
          title: `Reset ${groupName} nodes`,
          task: () => {
            const resetTasks = configGroup.map((config) => ({
              title: `Reset ${config.getName()} node`,
              task: () => resetNodeTask(config),
            }));

            return new Listr(resetTasks);
          },
        },
        {
          enabled: (ctx) => ctx.isHardReset && groupName === PRESET_LOCAL,
          title: 'Delete node configs',
          task: () => (
            configGroup
              .filter((config) => configFile.isConfigExists(config.getName()))
              .forEach((config) => configFile.removeConfig(config.getName()))
          ),
        },
        {
          enabled: (ctx) => !ctx.isHardReset
            && !ctx.isPlatformOnlyReset && groupName === PRESET_LOCAL,
          title: 'Configure Core nodes',
          task: () => configureCoreTask(configGroup),
        },
        {
          enabled: (ctx) => !ctx.isHardReset && groupName === PRESET_LOCAL,
          title: 'Configure Tenderdash nodes',
          task: () => configureTenderdashTask(configGroup),
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
      },
    );

    try {
      await tasks.run({
        isHardReset,
        isForce,
        isPlatformOnlyReset,
        isVerbose,
      });
    } catch (e) {
      throw new MuteOneLineError(e);
    }
  }
}

GroupResetCommand.description = 'Reset group nodes';

GroupResetCommand.flags = {
  ...GroupBaseCommand.flags,
  hard: Flags.boolean({
    description: 'reset config as well as data',
    default: false,
  }),
  force: Flags.boolean({
    char: 'f',
    description: 'reset even running node',
    default: false,
  }),
  platform: Flags.boolean({
    char: 'p',
    description: 'reset platform services and data only',
    default: false,
  }),
};

module.exports = GroupResetCommand;
