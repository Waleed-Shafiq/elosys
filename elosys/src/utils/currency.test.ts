/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
import { CurrencyUtils } from './currency'

describe('CurrencyUtils', () => {
  it('encode', () => {
    expect(CurrencyUtils.encode(0n)).toEqual('0')
    expect(CurrencyUtils.encode(1n)).toEqual('1')
    expect(CurrencyUtils.encode(100n)).toEqual('100')
    expect(CurrencyUtils.encode(10000n)).toEqual('10000')
    expect(CurrencyUtils.encode(100000000n)).toEqual('100000000')
  })

  it('decode', () => {
    expect(CurrencyUtils.decode('0')).toEqual(0n)
    expect(CurrencyUtils.decode('1')).toEqual(1n)
    expect(CurrencyUtils.decode('100')).toEqual(100n)
    expect(CurrencyUtils.decode('10000')).toEqual(10000n)
    expect(CurrencyUtils.decode('100000000')).toEqual(100000000n)
  })

  it('encodeElosys', () => {
    expect(CurrencyUtils.encodeElosys(0n)).toEqual('0.0')
    expect(CurrencyUtils.encodeElosys(1n)).toEqual('0.00000001')
    expect(CurrencyUtils.encodeElosys(100n)).toEqual('0.000001')
    expect(CurrencyUtils.encodeElosys(10000n)).toEqual('0.0001')
    expect(CurrencyUtils.encodeElosys(100000000n)).toEqual('1.0')

    expect(CurrencyUtils.encodeElosys(2394n)).toBe('0.00002394')
    expect(CurrencyUtils.encodeElosys(999n)).toBe('0.00000999')
  })

  it('decodeElosys', () => {
    expect(CurrencyUtils.decodeElosys('0.0')).toEqual(0n)
    expect(CurrencyUtils.decodeElosys('0.00000001')).toEqual(1n)
    expect(CurrencyUtils.decodeElosys('0.000001')).toEqual(100n)
    expect(CurrencyUtils.decodeElosys('0.0001')).toEqual(10000n)
    expect(CurrencyUtils.decodeElosys('1.0')).toEqual(100000000n)

    expect(CurrencyUtils.decodeElosys('0.00002394')).toBe(2394n)
    expect(CurrencyUtils.decodeElosys('0.00000999')).toBe(999n)
  })

  it('renderElosys', () => {
    expect(CurrencyUtils.renderElosys(0n)).toEqual('0.00000000')
    expect(CurrencyUtils.renderElosys(1n)).toEqual('0.00000001')
    expect(CurrencyUtils.renderElosys(100n)).toEqual('0.00000100')
    expect(CurrencyUtils.renderElosys(10000n)).toEqual('0.00010000')
    expect(CurrencyUtils.renderElosys(100000000n)).toEqual('1.00000000')
    expect(CurrencyUtils.renderElosys(1n, true)).toEqual('$ELOSYS 0.00000001')
  })

  it('renderOre', () => {
    expect(CurrencyUtils.renderOre(0n)).toEqual('0')
    expect(CurrencyUtils.renderOre(1n)).toEqual('1')
    expect(CurrencyUtils.renderOre(100n)).toEqual('100')
    expect(CurrencyUtils.renderOre(10000n)).toEqual('10000')
    expect(CurrencyUtils.renderOre(100000000n)).toEqual('100000000')
    expect(CurrencyUtils.renderOre(1n, true)).toEqual('$ORE 1')
  })
})
