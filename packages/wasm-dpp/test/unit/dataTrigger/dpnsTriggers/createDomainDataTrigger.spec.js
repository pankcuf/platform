const createDomainDataTrigger = require('@dashevo/dpp/lib/dataTrigger/dpnsTriggers/createDomainDataTrigger');

const DataTriggerExecutionContext = require('@dashevo/dpp/lib/dataTrigger/DataTriggerExecutionContext');
const DataTriggerExecutionResult = require('@dashevo/dpp/lib/dataTrigger/DataTriggerExecutionResult');

const { getParentDocumentFixture, getChildDocumentFixture, getTopDocumentFixture } = require('@dashevo/dpp/lib/test/fixtures/getDpnsDocumentFixture');
const getPreorderDocumentFixture = require('@dashevo/dpp/lib/test/fixtures/getPreorderDocumentFixture');
const getDpnsContractFixture = require('@dashevo/dpp/lib/test/fixtures/getDpnsContractFixture');
const getDocumentTransitionFixture = require('@dashevo/dpp/lib/test/fixtures/getDocumentTransitionsFixture');
const createStateRepositoryMock = require('@dashevo/dpp/lib/test/mocks/createStateRepositoryMock');

const { hash } = require('@dashevo/dpp/lib/util/hash');

const DataTriggerConditionError = require('@dashevo/dpp/lib/errors/consensus/state/dataContract/dataTrigger/DataTriggerConditionError');
const Identifier = require('@dashevo/dpp/lib/identifier/Identifier');
const StateTransitionExecutionContext = require('@dashevo/dpp/lib/stateTransition/StateTransitionExecutionContext');

describe('createDomainDataTrigger', () => {
  let parentDocumentTransition;
  let childDocumentTransition;
  let childDocument;
  let parentDocument;
  let topDocument;
  let context;
  let stateRepositoryMock;
  let dataContract;
  let topLevelIdentity;
  let executionContext;

  beforeEach(function beforeEach() {
    dataContract = getDpnsContractFixture();

    topDocument = getTopDocumentFixture();
    parentDocument = getParentDocumentFixture();
    childDocument = getChildDocumentFixture();
    const preorderDocument = getPreorderDocumentFixture();

    [parentDocumentTransition] = getDocumentTransitionFixture({
      create: [parentDocument],
    });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const {
      preorderSalt, records, normalizedParentDomainName, normalizedLabel,
    } = childDocument.getData();

    let fullDomainName = normalizedLabel;
    if (normalizedParentDomainName.length > 0) {
      fullDomainName = `${normalizedLabel}.${normalizedParentDomainName}`;
    }

    stateRepositoryMock = createStateRepositoryMock(this.sinonSandbox);
    stateRepositoryMock.fetchDocuments.resolves([]);

    const [normalizedParentLabel] = normalizedParentDomainName.split('.');
    const normalizedGrandParentDomainName = normalizedParentDomainName.split('.')
      .slice(1)
      .join('.');

    stateRepositoryMock.fetchDocuments
      .withArgs(
        dataContract.getId(),
        childDocument.getType(),
        {
          where: [
            ['normalizedParentDomainName', '==', normalizedGrandParentDomainName],
            ['normalizedLabel', '==', normalizedParentLabel],
          ],
        },
      )
      .resolves([parentDocument]);

    const saltedDomainHashBuffer = Buffer.concat([
      preorderSalt,
      Buffer.from(fullDomainName),
    ]);

    const saltedDomainHash = hash(saltedDomainHashBuffer);

    stateRepositoryMock.fetchDocuments
      .withArgs(
        dataContract.getId(),
        'preorder',
        { where: [['saltedDomainHash', '==', saltedDomainHash]] },
      )
      .resolves([preorderDocument.toObject()]);

    stateRepositoryMock.fetchTransaction.resolves(null);

    stateRepositoryMock.fetchTransaction
      .withArgs(
        records.dashUniqueIdentityId,
      )
      .resolves({ confirmations: 10 });

    executionContext = new StateTransitionExecutionContext();

    context = new DataTriggerExecutionContext(
      stateRepositoryMock,
      records.dashUniqueIdentityId,
      dataContract,
      executionContext,
    );

    topLevelIdentity = context.getOwnerId();
  });

  it('should successfully execute if document is valid', async () => {
    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result.isOk()).to.be.true();
  });

  it('should fail with invalid normalizedLabel', async () => {
    childDocument = getChildDocumentFixture({ normalizedLabel: childDocument.getData().label });
    stateRepositoryMock.fetchTransaction
      .withArgs(
        childDocument.getData().records.dashUniqueIdentityId,
      )
      .resolves({ confirmations: 10 });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal('Normalized label doesn\'t match label');
  });

  it('should fail with invalid parent domain', async () => {
    childDocument = getChildDocumentFixture({
      label: 'label',
      normalizedLabel: 'label',
      normalizedParentDomainName: 'parent.invalidname',
    });

    stateRepositoryMock.fetchTransaction
      .withArgs(
        childDocument.getData().records.dashUniqueIdentityId,
      )
      .resolves({ confirmations: 10 });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal('Parent domain is not present');

    expect(stateRepositoryMock.fetchDocuments).to.have.been.calledOnceWithExactly(
      context.getDataContract().getId(),
      'domain',
      {
        where: [
          ['normalizedParentDomainName', '==', 'invalidname'],
          ['normalizedLabel', '==', 'parent'],
        ],
      },
      executionContext,
    );
  });

  it('should fail with invalid dashUniqueIdentityId', async () => {
    const dashUniqueIdentityId = Identifier.from(
      Buffer.alloc(32, 5),
    );

    childDocument = getChildDocumentFixture({
      records: {
        dashUniqueIdentityId: dashUniqueIdentityId.toBuffer(),
      },
    });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal(`ownerId ${childDocument.getOwnerId()} doesn't match dashUniqueIdentityId ${dashUniqueIdentityId}`);
  });

  it('should fail with invalid dashAliasIdentityId', async () => {
    const dashUniqueIdentityId = Identifier.from(
      Buffer.alloc(32, 2),
    );

    childDocument = getChildDocumentFixture({
      records: {
        dashAliasIdentityId: dashUniqueIdentityId.toBuffer(),
      },
    });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal(`ownerId ${childDocument.getOwnerId()} doesn't match dashAliasIdentityId ${dashUniqueIdentityId}`);
  });

  it('should fail with preorder document was not found', async () => {
    childDocument = getChildDocumentFixture({
      preorderSalt: Buffer.alloc(256, '012fd'),
    });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal('preorderDocument was not found');
  });

  it('should fail with invalid full domain name length', async () => {
    childDocument = getChildDocumentFixture({
      normalizedParentDomainName: 'a'.repeat(512),
    });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal(
      'Full domain name length can not be more than 253 characters long but got 518',
    );
  });

  it('should fail with identity can\'t create top level domain', async () => {
    parentDocumentTransition.data.normalizedParentDomainName = '';

    topLevelIdentity = Buffer.from('someIdentity');

    const result = await createDomainDataTrigger(
      parentDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal(
      'Can\'t create top level domain for this identity',
    );
  });

  it('should fail with disallowed domain creation', async () => {
    parentDocument.ownerId = Buffer.from('newId');

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();
    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal(
      'The subdomain can be created only by the parent domain owner',
    );
  });

  it('should fail with allowing subdomains for non top level domain', async () => {
    childDocument = getChildDocumentFixture({ subdomainRules: { allowSubdomains: true } });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.false();

    const [error] = result.getErrors();

    expect(error).to.be.an.instanceOf(DataTriggerConditionError);
    expect(error.message).to.equal(
      'Allowing subdomains registration is forbidden for non top level domains',
    );
  });

  it('should allow creating a second level domain by any identity', async () => {
    topDocument.ownerId = 'anotherId';

    stateRepositoryMock.fetchDocuments.resolves([topDocument]);

    const result = await createDomainDataTrigger(
      parentDocumentTransition, context, topLevelIdentity,
    );

    expect(result.isOk()).to.be.true();
  });

  it('should return DataTriggerExecutionResult om dry run', async () => {
    context.getStateTransitionExecutionContext().enableDryRun();

    childDocument = getChildDocumentFixture({ normalizedLabel: childDocument.getData().label });
    stateRepositoryMock.fetchTransaction
      .withArgs(
        childDocument.getData().records.dashUniqueIdentityId,
      )
      .resolves({ confirmations: 10 });

    [childDocumentTransition] = getDocumentTransitionFixture({
      create: [childDocument],
    });

    const result = await createDomainDataTrigger(
      childDocumentTransition, context, topLevelIdentity,
    );

    context.getStateTransitionExecutionContext().disableDryRun();

    expect(result).to.be.an.instanceOf(DataTriggerExecutionResult);
    expect(result.isOk()).to.be.true();
  });
});
