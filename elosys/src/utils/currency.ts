/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

import { formatFixed, parseFixed } from '@ethersproject/bignumber'
import { isNativeIdentifier } from './asset'
import { BigIntUtils } from './bigint'
import { ErrorUtils } from './error'
import { FixedNumberUtils } from './fixedNumber'

export class CurrencyUtils {
  static locale?: string

  /**
   * Serializes ore as elosys with up to 8 decimal places
   */
  static encodeElosys(amount: bigint): string {
    return formatFixed(amount, 8)
  }

  /**
   * Parses elosys into ore
   */
  static decodeElosys(amount: string | number): bigint {
    return parseFixed(amount.toString(), 8).toBigInt()
  }

  /**
   * Parses Elosys into ore but returns the error if parsing fails
   */
  static decodeElosysTry(amount: string | number): [bigint, null] | [null, ParseFixedError] {
    try {
      const parsed = parseFixed(amount.toString(), 8).toBigInt()
      return [parsed, null]
    } catch (e) {
      if (isParseFixedError(e)) {
        return [null, e]
      }
      throw e
    }
  }

  /**
   * Deserialize ore back into bigint
   */
  static decode(amount: string): bigint {
    return BigInt(amount)
  }

  static decodeTry(amount: string): [bigint, null] | [null, Error] {
    return BigIntUtils.tryParse(amount)
  }

  /**
   * Serialize ore into a string
   */
  static encode(amount: bigint): string {
    return amount.toString()
  }

  /*
   * Renders ore as elosys for human-readable purposes
   */
  static renderElosys(amount: bigint | string, includeTicker = false, assetId?: string): string {
    if (typeof amount === 'string') {
      amount = this.decode(amount)
    }

    const elosys = FixedNumberUtils.render(amount, 8)

    if (includeTicker) {
      let ticker = '$ELOSYS'
      if (assetId && !isNativeIdentifier(assetId)) {
        ticker = assetId
      }
      return `${ticker} ${elosys}`
    }

    return elosys
  }

  /*
   * Renders ore for human-readable purposes
   */
  static renderOre(amount: bigint | string, includeTicker = false, assetId?: string): string {
    if (typeof amount === 'string') {
      amount = this.decode(amount)
    }

    const ore = amount.toString()

    if (includeTicker) {
      let ticker = '$ORE'
      if (assetId && !isNativeIdentifier(assetId)) {
        ticker = assetId
      }
      return `${ticker} ${ore}`
    }

    return ore
  }
}

export interface ParseFixedError extends Error {
  code: 'INVALID_ARGUMENT' | 'NUMERIC_FAULT'
  reason: string
}

export function isParseFixedError(error: unknown): error is ParseFixedError {
  return (
    ErrorUtils.isNodeError(error) &&
    (error['code'] === 'INVALID_ARGUMENT' || error['code'] === 'NUMERIC_FAULT') &&
    'reason' in error &&
    typeof error['reason'] === 'string'
  )
}

export const ORE_TO_ELOSYS = 100000000
export const MINIMUM_ORE_AMOUNT = 0n
export const MAXIMUM_ORE_AMOUNT = 2n ** 64n
export const MINIMUM_ELOSYS_AMOUNT = CurrencyUtils.renderElosys(MINIMUM_ORE_AMOUNT)
export const MAXIMUM_ELOSYS_AMOUNT = CurrencyUtils.renderElosys(MAXIMUM_ORE_AMOUNT)
