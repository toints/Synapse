/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/web3.js';
import { RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const INITIALIZE_OPERATOR_SNAPSHOT_DISCRIMINATOR = 9;

export function getInitializeOperatorSnapshotDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_OPERATOR_SNAPSHOT_DISCRIMINATOR);
}

export type InitializeOperatorSnapshotInstruction<
  TProgram extends string = typeof RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountOperator extends string | IAccountMeta<string> = string,
  TAccountNcnOperatorState extends string | IAccountMeta<string> = string,
  TAccountEpochSnapshot extends string | IAccountMeta<string> = string,
  TAccountOperatorSnapshot extends string | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountRestakingProgram extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountOperator extends string
        ? ReadonlyAccount<TAccountOperator>
        : TAccountOperator,
      TAccountNcnOperatorState extends string
        ? ReadonlyAccount<TAccountNcnOperatorState>
        : TAccountNcnOperatorState,
      TAccountEpochSnapshot extends string
        ? WritableAccount<TAccountEpochSnapshot>
        : TAccountEpochSnapshot,
      TAccountOperatorSnapshot extends string
        ? WritableAccount<TAccountOperatorSnapshot>
        : TAccountOperatorSnapshot,
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountRestakingProgram extends string
        ? ReadonlyAccount<TAccountRestakingProgram>
        : TAccountRestakingProgram,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeOperatorSnapshotInstructionData = {
  discriminator: number;
  epoch: bigint;
};

export type InitializeOperatorSnapshotInstructionDataArgs = {
  epoch: number | bigint;
};

export function getInitializeOperatorSnapshotInstructionDataEncoder(): Encoder<InitializeOperatorSnapshotInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['epoch', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: INITIALIZE_OPERATOR_SNAPSHOT_DISCRIMINATOR,
    })
  );
}

export function getInitializeOperatorSnapshotInstructionDataDecoder(): Decoder<InitializeOperatorSnapshotInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['epoch', getU64Decoder()],
  ]);
}

export function getInitializeOperatorSnapshotInstructionDataCodec(): Codec<
  InitializeOperatorSnapshotInstructionDataArgs,
  InitializeOperatorSnapshotInstructionData
> {
  return combineCodec(
    getInitializeOperatorSnapshotInstructionDataEncoder(),
    getInitializeOperatorSnapshotInstructionDataDecoder()
  );
}

export type InitializeOperatorSnapshotInput<
  TAccountConfig extends string = string,
  TAccountNcn extends string = string,
  TAccountOperator extends string = string,
  TAccountNcnOperatorState extends string = string,
  TAccountEpochSnapshot extends string = string,
  TAccountOperatorSnapshot extends string = string,
  TAccountPayer extends string = string,
  TAccountRestakingProgram extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  config: Address<TAccountConfig>;
  ncn: Address<TAccountNcn>;
  operator: Address<TAccountOperator>;
  ncnOperatorState: Address<TAccountNcnOperatorState>;
  epochSnapshot: Address<TAccountEpochSnapshot>;
  operatorSnapshot: Address<TAccountOperatorSnapshot>;
  payer: TransactionSigner<TAccountPayer>;
  restakingProgram: Address<TAccountRestakingProgram>;
  systemProgram?: Address<TAccountSystemProgram>;
  epoch: InitializeOperatorSnapshotInstructionDataArgs['epoch'];
};

export function getInitializeOperatorSnapshotInstruction<
  TAccountConfig extends string,
  TAccountNcn extends string,
  TAccountOperator extends string,
  TAccountNcnOperatorState extends string,
  TAccountEpochSnapshot extends string,
  TAccountOperatorSnapshot extends string,
  TAccountPayer extends string,
  TAccountRestakingProgram extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS,
>(
  input: InitializeOperatorSnapshotInput<
    TAccountConfig,
    TAccountNcn,
    TAccountOperator,
    TAccountNcnOperatorState,
    TAccountEpochSnapshot,
    TAccountOperatorSnapshot,
    TAccountPayer,
    TAccountRestakingProgram,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeOperatorSnapshotInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountNcn,
  TAccountOperator,
  TAccountNcnOperatorState,
  TAccountEpochSnapshot,
  TAccountOperatorSnapshot,
  TAccountPayer,
  TAccountRestakingProgram,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    ncn: { value: input.ncn ?? null, isWritable: false },
    operator: { value: input.operator ?? null, isWritable: false },
    ncnOperatorState: {
      value: input.ncnOperatorState ?? null,
      isWritable: false,
    },
    epochSnapshot: { value: input.epochSnapshot ?? null, isWritable: true },
    operatorSnapshot: {
      value: input.operatorSnapshot ?? null,
      isWritable: true,
    },
    payer: { value: input.payer ?? null, isWritable: true },
    restakingProgram: {
      value: input.restakingProgram ?? null,
      isWritable: false,
    },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.operator),
      getAccountMeta(accounts.ncnOperatorState),
      getAccountMeta(accounts.epochSnapshot),
      getAccountMeta(accounts.operatorSnapshot),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.restakingProgram),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeOperatorSnapshotInstructionDataEncoder().encode(
      args as InitializeOperatorSnapshotInstructionDataArgs
    ),
  } as InitializeOperatorSnapshotInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountNcn,
    TAccountOperator,
    TAccountNcnOperatorState,
    TAccountEpochSnapshot,
    TAccountOperatorSnapshot,
    TAccountPayer,
    TAccountRestakingProgram,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeOperatorSnapshotInstruction<
  TProgram extends string = typeof RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    ncn: TAccountMetas[1];
    operator: TAccountMetas[2];
    ncnOperatorState: TAccountMetas[3];
    epochSnapshot: TAccountMetas[4];
    operatorSnapshot: TAccountMetas[5];
    payer: TAccountMetas[6];
    restakingProgram: TAccountMetas[7];
    systemProgram: TAccountMetas[8];
  };
  data: InitializeOperatorSnapshotInstructionData;
};

export function parseInitializeOperatorSnapshotInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeOperatorSnapshotInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 9) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      ncn: getNextAccount(),
      operator: getNextAccount(),
      ncnOperatorState: getNextAccount(),
      epochSnapshot: getNextAccount(),
      operatorSnapshot: getNextAccount(),
      payer: getNextAccount(),
      restakingProgram: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeOperatorSnapshotInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
