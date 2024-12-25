/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  type Codec,
  type Decoder,
  type Encoder,
  type ReadonlyUint8Array,
} from '@solana/web3.js';

export type MerkleRoot = { root: ReadonlyUint8Array };

export type MerkleRootArgs = MerkleRoot;

export function getMerkleRootEncoder(): Encoder<MerkleRootArgs> {
  return getStructEncoder([['root', fixEncoderSize(getBytesEncoder(), 32)]]);
}

export function getMerkleRootDecoder(): Decoder<MerkleRoot> {
  return getStructDecoder([['root', fixDecoderSize(getBytesDecoder(), 32)]]);
}

export function getMerkleRootCodec(): Codec<MerkleRootArgs, MerkleRoot> {
  return combineCodec(getMerkleRootEncoder(), getMerkleRootDecoder());
}