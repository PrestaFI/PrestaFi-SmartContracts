import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CD5METAIMBTCDMB6JOZO63B4XWYMJVNDFWE4YK4I4CJED7H6IOEQYNJT",
  }
} as const

export type DataKey = {tag: "AssignedCreditMap", values: readonly [string, string]} | {tag: "UsedCreditMap", values: readonly [string, string]} | {tag: "PaymentsPaidMap", values: readonly [string, i128]};

export const Errors = {

}

export interface Client {
  /**
   * Construct and simulate a mint_usdc transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  mint_usdc: ({contract_address, address_to_mint, amount_to_mint}: {contract_address: string, address_to_mint: string, amount_to_mint: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAAAwAAAAEAAAAAAAAAEUFzc2lnbmVkQ3JlZGl0TWFwAAAAAAAAAgAAABMAAAATAAAAAQAAAAAAAAANVXNlZENyZWRpdE1hcAAAAAAAAAIAAAATAAAAEwAAAAEAAAAAAAAAD1BheW1lbnRzUGFpZE1hcAAAAAACAAAAEwAAAAs=",
        "AAAAAAAAAAAAAAAJbWludF91c2RjAAAAAAAAAwAAAAAAAAAQY29udHJhY3RfYWRkcmVzcwAAABMAAAAAAAAAD2FkZHJlc3NfdG9fbWludAAAAAATAAAAAAAAAA5hbW91bnRfdG9fbWludAAAAAAABAAAAAA=" ]),
      options
    )
  }
  public readonly fromJSON = {
    mint_usdc: this.txFromJSON<null>
  }
}