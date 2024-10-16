const { Listr } = require('listr2');

const { Flags } = require('@oclif/core');

const ConfigBaseCommand = require('../oclif/command/ConfigBaseCommand');

const MuteOneLineError = require('../oclif/errors/MuteOneLineError');

class StartCommand extends ConfigBaseCommand {
  /**
   * @param {Object} args
   * @param {Object} flags
   * @param {DockerCompose} dockerCompose
   * @param {startNodeTask} startNodeTask
   * @param {waitForNodeToBeReadyTask} waitForNodeToBeReadyTask
   * @param {Config} config
   * @return {Promise<void>}
   */
  async runWithDependencies(
    args,
    {
      'wait-for-readiness': waitForReadiness,
      verbose: isVerbose,
      platform: platformOnly,
    },
    dockerCompose,
    startNodeTask,
    waitForNodeToBeReadyTask,
    config,
  ) {
    const tasks = new Listr(
      [
        {
          title: `Start ${config.getName()} node`,
          task: () => startNodeTask(config),
        },
        {
          title: 'Wait for nodes to be ready',
          enabled: () => waitForReadiness,
          task: () => waitForNodeToBeReadyTask(config),
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
        isVerbose,
        platformOnly: platformOnly === true,
      });
    } catch (e) {
      throw new MuteOneLineError(e);
    }
  }
}

StartCommand.description = 'Start node';

StartCommand.flags = {
  ...ConfigBaseCommand.flags,
  'wait-for-readiness': Flags.boolean({ char: 'w', description: 'wait for nodes to be ready', default: false }),
  platform: Flags.boolean({ char: 'p', description: 'start only platform', default: false }),
};

module.exports = StartCommand;
