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

export const ADMIN_SET_NEW_ADMIN_DISCRIMINATOR = 2;

export function getAdminSetNewAdminDiscriminatorBytes() {
  return getU8Encoder().encode(ADMIN_SET_NEW_ADMIN_DISCRIMINATOR);
}

export type AdminSetNewAdminInstruction<
  TProgram extends string = typeof NCN_PORTAL_PROGRAM_ADDRESS,
  TAccountWhitelistInfo extends string | IAccountMeta<string> = string,
  TAccountAdminInfo extends string | IAccountMeta<string> = string,
  TAccountNewAdminInfo extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountWhitelistInfo extends string
        ? WritableAccount<TAccountWhitelistInfo>
        : TAccountWhitelistInfo,
      TAccountAdminInfo extends string
        ? WritableSignerAccount<TAccountAdminInfo> &
            IAccountSignerMeta<TAccountAdminInfo>
        : TAccountAdminInfo,
      TAccountNewAdminInfo extends string
        ? ReadonlyAccount<TAccountNewAdminInfo>
        : TAccountNewAdminInfo,
      ...TRemainingAccounts,
    ]
  >;

export type AdminSetNewAdminInstructionData = { discriminator: number };

export type AdminSetNewAdminInstructionDataArgs = {};

export function getAdminSetNewAdminInstructionDataEncoder(): Encoder<AdminSetNewAdminInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: ADMIN_SET_NEW_ADMIN_DISCRIMINATOR })
  );
}

export function getAdminSetNewAdminInstructionDataDecoder(): Decoder<AdminSetNewAdminInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getAdminSetNewAdminInstructionDataCodec(): Codec<
  AdminSetNewAdminInstructionDataArgs,
  AdminSetNewAdminInstructionData
> {
  return combineCodec(
    getAdminSetNewAdminInstructionDataEncoder(),
    getAdminSetNewAdminInstructionDataDecoder()
  );
}

export type AdminSetNewAdminInput<
  TAccountWhitelistInfo extends string = string,
  TAccountAdminInfo extends string = string,
  TAccountNewAdminInfo extends string = string,
> = {
  whitelistInfo: Address<TAccountWhitelistInfo>;
  adminInfo: TransactionSigner<TAccountAdminInfo>;
  newAdminInfo: Address<TAccountNewAdminInfo>;
};

export function getAdminSetNewAdminInstruction<
  TAccountWhitelistInfo extends string,
  TAccountAdminInfo extends string,
  TAccountNewAdminInfo extends string,
  TProgramAddress extends Address = typeof NCN_PORTAL_PROGRAM_ADDRESS,
>(
  input: AdminSetNewAdminInput<
    TAccountWhitelistInfo,
    TAccountAdminInfo,
    TAccountNewAdminInfo
  >,
  config?: { programAddress?: TProgramAddress }
): AdminSetNewAdminInstruction<
  TProgramAddress,
  TAccountWhitelistInfo,
  TAccountAdminInfo,
  TAccountNewAdminInfo
> {
  // Program address.
  const programAddress = config?.programAddress ?? NCN_PORTAL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    whitelistInfo: { value: input.whitelistInfo ?? null, isWritable: true },
    adminInfo: { value: input.adminInfo ?? null, isWritable: true },
    newAdminInfo: { value: input.newAdminInfo ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.whitelistInfo),
      getAccountMeta(accounts.adminInfo),
      getAccountMeta(accounts.newAdminInfo),
    ],
    programAddress,
    data: getAdminSetNewAdminInstructionDataEncoder().encode({}),
  } as AdminSetNewAdminInstruction<
    TProgramAddress,
    TAccountWhitelistInfo,
    TAccountAdminInfo,
    TAccountNewAdminInfo
  >;

  return instruction;
}

export type ParsedAdminSetNewAdminInstruction<
  TProgram extends string = typeof NCN_PORTAL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    whitelistInfo: TAccountMetas[0];
    adminInfo: TAccountMetas[1];
    newAdminInfo: TAccountMetas[2];
  };
  data: AdminSetNewAdminInstructionData;
};

export function parseAdminSetNewAdminInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAdminSetNewAdminInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
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
      whitelistInfo: getNextAccount(),
      adminInfo: getNextAccount(),
      newAdminInfo: getNextAccount(),
    },
    data: getAdminSetNewAdminInstructionDataDecoder().decode(instruction.data),
  };
}