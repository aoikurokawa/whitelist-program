/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  getAddressDecoder,
  getAddressEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
} from '@solana/web3.js';

export type Whitelist = { discriminator: bigint; admin: Address };

export type WhitelistArgs = { discriminator: number | bigint; admin: Address };

export function getWhitelistEncoder(): Encoder<WhitelistArgs> {
  return getStructEncoder([
    ['discriminator', getU64Encoder()],
    ['admin', getAddressEncoder()],
  ]);
}

export function getWhitelistDecoder(): Decoder<Whitelist> {
  return getStructDecoder([
    ['discriminator', getU64Decoder()],
    ['admin', getAddressDecoder()],
  ]);
}

export function getWhitelistCodec(): Codec<WhitelistArgs, Whitelist> {
  return combineCodec(getWhitelistEncoder(), getWhitelistDecoder());
}

export function decodeWhitelist<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<Whitelist, TAddress>;
export function decodeWhitelist<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<Whitelist, TAddress>;
export function decodeWhitelist<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<Whitelist, TAddress> | MaybeAccount<Whitelist, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getWhitelistDecoder()
  );
}

export async function fetchWhitelist<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<Whitelist, TAddress>> {
  const maybeAccount = await fetchMaybeWhitelist(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeWhitelist<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<Whitelist, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeWhitelist(maybeAccount);
}

export async function fetchAllWhitelist(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<Whitelist>[]> {
  const maybeAccounts = await fetchAllMaybeWhitelist(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeWhitelist(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<Whitelist>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeWhitelist(maybeAccount));
}

export function getWhitelistSize(): number {
  return 32;
}