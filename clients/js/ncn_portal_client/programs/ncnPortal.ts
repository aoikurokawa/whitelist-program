/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  containsBytes,
  getU8Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/web3.js';
import {
  type ParsedAddToWhitelistInstruction,
  type ParsedAdminUpdateMerkleRootInstruction,
  type ParsedCheckWhitelistedInstruction,
  type ParsedInitializeWhitelistInstruction,
  type ParsedRemoveFromWhitelistInstruction,
} from '../instructions';

export const NCN_PORTAL_PROGRAM_ADDRESS =
  'DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr' as Address<'DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr'>;

export enum NcnPortalAccount {
  Whitelist,
  WhitelistEntry,
}

export enum NcnPortalInstruction {
  InitializeWhitelist,
  AdminUpdateMerkleRoot,
  AddToWhitelist,
  CheckWhitelisted,
  RemoveFromWhitelist,
}

export function identifyNcnPortalInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): NcnPortalInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return NcnPortalInstruction.InitializeWhitelist;
  }
  if (containsBytes(data, getU8Encoder().encode(1), 0)) {
    return NcnPortalInstruction.AdminUpdateMerkleRoot;
  }
  if (containsBytes(data, getU8Encoder().encode(2), 0)) {
    return NcnPortalInstruction.AddToWhitelist;
  }
  if (containsBytes(data, getU8Encoder().encode(3), 0)) {
    return NcnPortalInstruction.CheckWhitelisted;
  }
  if (containsBytes(data, getU8Encoder().encode(4), 0)) {
    return NcnPortalInstruction.RemoveFromWhitelist;
  }
  throw new Error(
    'The provided instruction could not be identified as a ncnPortal instruction.'
  );
}

export type ParsedNcnPortalInstruction<
  TProgram extends string = 'DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr',
> =
  | ({
      instructionType: NcnPortalInstruction.InitializeWhitelist;
    } & ParsedInitializeWhitelistInstruction<TProgram>)
  | ({
      instructionType: NcnPortalInstruction.AdminUpdateMerkleRoot;
    } & ParsedAdminUpdateMerkleRootInstruction<TProgram>)
  | ({
      instructionType: NcnPortalInstruction.AddToWhitelist;
    } & ParsedAddToWhitelistInstruction<TProgram>)
  | ({
      instructionType: NcnPortalInstruction.CheckWhitelisted;
    } & ParsedCheckWhitelistedInstruction<TProgram>)
  | ({
      instructionType: NcnPortalInstruction.RemoveFromWhitelist;
    } & ParsedRemoveFromWhitelistInstruction<TProgram>);
