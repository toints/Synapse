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

export const INITIALIZE_WEIGHT_TABLE_DISCRIMINATOR = 5;

export function getInitializeWeightTableDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_WEIGHT_TABLE_DISCRIMINATOR);
}

export type InitializeWeightTableInstruction<
  TProgram extends string = typeof RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS,
  TAccountVaultRegistry extends string | IAccountMeta<string> = string,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountWeightTable extends string | IAccountMeta<string> = string,
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
      TAccountVaultRegistry extends string
        ? ReadonlyAccount<TAccountVaultRegistry>
        : TAccountVaultRegistry,
      TAccountNcn extends string ? ReadonlyAccount<TAccountNcn> : TAccountNcn,
      TAccountWeightTable extends string
        ? WritableAccount<TAccountWeightTable>
        : TAccountWeightTable,
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

export type InitializeWeightTableInstructionData = {
  discriminator: number;
  epoch: bigint;
};

export type InitializeWeightTableInstructionDataArgs = {
  epoch: number | bigint;
};

export function getInitializeWeightTableInstructionDataEncoder(): Encoder<InitializeWeightTableInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['epoch', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: INITIALIZE_WEIGHT_TABLE_DISCRIMINATOR,
    })
  );
}

export function getInitializeWeightTableInstructionDataDecoder(): Decoder<InitializeWeightTableInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['epoch', getU64Decoder()],
  ]);
}

export function getInitializeWeightTableInstructionDataCodec(): Codec<
  InitializeWeightTableInstructionDataArgs,
  InitializeWeightTableInstructionData
> {
  return combineCodec(
    getInitializeWeightTableInstructionDataEncoder(),
    getInitializeWeightTableInstructionDataDecoder()
  );
}

export type InitializeWeightTableInput<
  TAccountVaultRegistry extends string = string,
  TAccountNcn extends string = string,
  TAccountWeightTable extends string = string,
  TAccountPayer extends string = string,
  TAccountRestakingProgram extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  vaultRegistry: Address<TAccountVaultRegistry>;
  ncn: Address<TAccountNcn>;
  weightTable: Address<TAccountWeightTable>;
  payer: TransactionSigner<TAccountPayer>;
  restakingProgram: Address<TAccountRestakingProgram>;
  systemProgram?: Address<TAccountSystemProgram>;
  epoch: InitializeWeightTableInstructionDataArgs['epoch'];
};

export function getInitializeWeightTableInstruction<
  TAccountVaultRegistry extends string,
  TAccountNcn extends string,
  TAccountWeightTable extends string,
  TAccountPayer extends string,
  TAccountRestakingProgram extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS,
>(
  input: InitializeWeightTableInput<
    TAccountVaultRegistry,
    TAccountNcn,
    TAccountWeightTable,
    TAccountPayer,
    TAccountRestakingProgram,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeWeightTableInstruction<
  TProgramAddress,
  TAccountVaultRegistry,
  TAccountNcn,
  TAccountWeightTable,
  TAccountPayer,
  TAccountRestakingProgram,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    vaultRegistry: { value: input.vaultRegistry ?? null, isWritable: false },
    ncn: { value: input.ncn ?? null, isWritable: false },
    weightTable: { value: input.weightTable ?? null, isWritable: true },
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
      getAccountMeta(accounts.vaultRegistry),
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.weightTable),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.restakingProgram),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeWeightTableInstructionDataEncoder().encode(
      args as InitializeWeightTableInstructionDataArgs
    ),
  } as InitializeWeightTableInstruction<
    TProgramAddress,
    TAccountVaultRegistry,
    TAccountNcn,
    TAccountWeightTable,
    TAccountPayer,
    TAccountRestakingProgram,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeWeightTableInstruction<
  TProgram extends string = typeof RELAYER_NCN_PROGRAM_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    vaultRegistry: TAccountMetas[0];
    ncn: TAccountMetas[1];
    weightTable: TAccountMetas[2];
    payer: TAccountMetas[3];
    restakingProgram: TAccountMetas[4];
    systemProgram: TAccountMetas[5];
  };
  data: InitializeWeightTableInstructionData;
};

export function parseInitializeWeightTableInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeWeightTableInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 6) {
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
      vaultRegistry: getNextAccount(),
      ncn: getNextAccount(),
      weightTable: getNextAccount(),
      payer: getNextAccount(),
      restakingProgram: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeWeightTableInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
