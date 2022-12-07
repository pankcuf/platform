const {
  tendermint: {
    abci: {
      ResponseProcessProposal,
    },
  },
} = require('@dashevo/abci/types');

const statuses = require('./proposal/statuses');

/**
 * @param {BaseLogger} logger
 * @param {verifyChainLock} verifyChainLock
 * @param {processProposal} processProposal
 * @param {BlockExecutionContext} proposalBlockExecutionContext
 * @return {processProposalHandler}
 */
function processProposalHandlerFactory(
  logger,
  verifyChainLock,
  processProposal,
  proposalBlockExecutionContext,
) {
  /**
   * @typedef processProposalHandler
   * @param {abci.RequestProcessProposal} request
   * @return {Promise<abci.ResponseProcessProposal>}
   */
  async function processProposalHandler(request) {
    const {
      height,
      coreChainLockUpdate,
      round,
    } = request;

    const consensusLogger = logger.child({
      height: height.toString(),
      round,
      abciMethod: 'processProposal',
    });

    consensusLogger.debug('ProcessProposal ABCI method requested');
    consensusLogger.trace({ abciRequest: request });

    // Skip process proposal if it was already prepared for this height and round
    const prepareProposalResult = proposalBlockExecutionContext.getPrepareProposalResult();

    if (prepareProposalResult
      && proposalBlockExecutionContext.getHeight().toNumber() === height.toNumber()
      && proposalBlockExecutionContext.getRound() === round) {
      consensusLogger.debug('Skip processing proposal and return prepared result');

      const {
        appHash,
        txResults,
        consensusParamUpdates,
        validatorSetUpdate,
      } = prepareProposalResult;

      return new ResponseProcessProposal({
        status: statuses.ACCEPT,
        appHash,
        txResults,
        consensusParamUpdates,
        validatorSetUpdate,
      });
    }

    if (coreChainLockUpdate) {
      const chainLockIsValid = await verifyChainLock(coreChainLockUpdate);

      if (!chainLockIsValid) {
        consensusLogger.warn({
          coreChainLockUpdate,
        }, `Block proposal #${height} round #${round} rejected due to invalid core chain locked height update`);

        return new ResponseProcessProposal({
          status: statuses.REJECT,
        });
      }

      logger.debug({
        coreChainLockUpdate,
      }, `ChainLock is valid for height ${coreChainLockUpdate.coreBlockHeight}`);
    }

    return processProposal(request, consensusLogger);
  }

  return processProposalHandler;
}

module.exports = processProposalHandlerFactory;
