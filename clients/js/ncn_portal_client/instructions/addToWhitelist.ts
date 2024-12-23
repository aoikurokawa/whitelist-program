/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
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
import { NCN_PORTAL_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const ADD_TO_WHITELIST_DISCRIMINATOR = 1;

export function getAddToWhitelistDiscriminatorBytes() {
  return getU8Encoder().encode(ADD_TO_WHITELIST_DISCRIMINATOR);
}

export type AddToWhitelistInstruction<
  TProgram extends string = typeof NCN_PORTAL_PROGRAM_ADDRESS,
  TAccountWhitelist extends string | IAccountMeta<string> = string,
  TAccountWhitelistEntry extends string | IAccountMeta<string> = string,
  TAccountWhitelisted extends string | IAccountMeta<string> = string,
  TAccountAdmin extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountWhitelist extends string
        ? ReadonlyAccount<TAccountWhitelist>
        : TAccountWhitelist,
      TAccountWhitelistEntry extends string
        ? WritableAccount<TAccountWhitelistEntry>
        : TAccountWhitelistEntry,
      TAccountWhitelisted extends string
        ? ReadonlyAccount<TAccountWhitelisted>
        : TAccountWhitelisted,
      TAccountAdmin extends string
        ? WritableSignerAccount<TAccountAdmin> &
            IAccountSignerMeta<TAccountAdmin>
        : TAccountAdmin,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type AddToWhitelistInstructionData = {
  discriminator: number;
  rateLimiting: bigint;
};

export type AddToWhitelistInstructionDataArgs = {
  rateLimiting: number | bigint;
};

export function getAddToWhitelistInstructionDataEncoder(): Encoder<AddToWhitelistInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['rateLimiting', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: ADD_TO_WHITELIST_DISCRIMINATOR })
  );
}

export function getAddToWhitelistInstructionDataDecoder(): Decoder<AddToWhitelistInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['rateLimiting', getU64Decoder()],
  ]);
}

export function getAddToWhitelistInstructionDataCodec(): Codec<
  AddToWhitelistInstructionDataArgs,
  AddToWhitelistInstructionData
> {
  return combineCodec(
    getAddToWhitelistInstructionDataEncoder(),
    getAddToWhitelistInstructionDataDecoder()
  );
}

export type AddToWhitelistInput<
  TAccountWhitelist extends string = string,
  TAccountWhitelistEntry extends string = string,
  TAccountWhitelisted extends string = string,
  TAccountAdmin extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  whitelist: Address<TAccountWhitelist>;
  whitelistEntry: Address<TAccountWhitelistEntry>;
  whitelisted: Address<TAccountWhitelisted>;
  admin: TransactionSigner<TAccountAdmin>;
  systemProgram?: Address<TAccountSystemProgram>;
  rateLimiting: AddToWhitelistInstructionDataArgs['rateLimiting'];
};

export function getAddToWhitelistInstruction<
  TAccountWhitelist extends string,
  TAccountWhitelistEntry extends string,
  TAccountWhitelisted extends string,
  TAccountAdmin extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof NCN_PORTAL_PROGRAM_ADDRESS,
>(
  input: AddToWhitelistInput<
    TAccountWhitelist,
    TAccountWhitelistEntry,
    TAccountWhitelisted,
    TAccountAdmin,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): AddToWhitelistInstruction<
  TProgramAddress,
  TAccountWhitelist,
  TAccountWhitelistEntry,
  TAccountWhitelisted,
  TAccountAdmin,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? NCN_PORTAL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    whitelist: { value: input.whitelist ?? null, isWritable: false },
    whitelistEntry: { value: input.whitelistEntry ?? null, isWritable: true },
    whitelisted: { value: input.whitelisted ?? null, isWritable: false },
    admin: { value: input.admin ?? null, isWritable: true },
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
      getAccountMeta(accounts.whitelist),
      getAccountMeta(accounts.whitelistEntry),
      getAccountMeta(accounts.whitelisted),
      getAccountMeta(accounts.admin),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getAddToWhitelistInstructionDataEncoder().encode(
      args as AddToWhitelistInstructionDataArgs
    ),
  } as AddToWhitelistInstruction<
    TProgramAddress,
    TAccountWhitelist,
    TAccountWhitelistEntry,
    TAccountWhitelisted,
    TAccountAdmin,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedAddToWhitelistInstruction<
  TProgram extends string = typeof NCN_PORTAL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    whitelist: TAccountMetas[0];
    whitelistEntry: TAccountMetas[1];
    whitelisted: TAccountMetas[2];
    admin: TAccountMetas[3];
    systemProgram: TAccountMetas[4];
  };
  data: AddToWhitelistInstructionData;
};

export function parseAddToWhitelistInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAddToWhitelistInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
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
      whitelist: getNextAccount(),
      whitelistEntry: getNextAccount(),
      whitelisted: getNextAccount(),
      admin: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getAddToWhitelistInstructionDataDecoder().decode(instruction.data),
  };
}
