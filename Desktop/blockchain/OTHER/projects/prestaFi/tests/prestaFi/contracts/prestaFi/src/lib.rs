#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, BytesN, Env, IntoVal, Vec, log, symbol_short, Symbol, map};



mod usdc_contract 
{
    soroban_sdk::contractimport!(file = "../../../usdc-mockup/target/wasm32-unknown-unknown/release/usdc_mockup.wasm");
}

mod lumen_contract 
{
    soroban_sdk::contractimport!(file = "../../../wrapped-lumen-mockup/target/wasm32-unknown-unknown/release/wrapped_lumen_mockup.wasm");
}



const PRICE: Symbol = symbol_short!("PRICE");



#[derive(Clone)]
#[contracttype]
pub enum DataKey 
{
    AssignedCreditMap(Address, Address),    //u32
    UsedCreditMap(Address, Address),        //u32
    PaymentsWithInterests(Address),          //u32
    PaymentsPaidMap(Address, i128),         //bool
}

#[contract]
pub struct PrestaFiContract;

#[contractimpl]
impl PrestaFiContract 
{
    pub fn set_lumen_price(env: Env, lumen_price: u32)
    {
        env.storage().instance().set(&PRICE, &lumen_price);
    }

    pub fn get_lumen_price(env: Env) -> u32
    {
        let price: u32 = env.storage().instance().get(&PRICE).unwrap_or(0);

        price
    }

    pub fn assign_collateral(env: Env, lumen_contract_address: Address, amount_to_assign: u32, address_to_assign: Address, caller: Address)
    {
        caller.require_auth();

        let client = lumen_contract::Client::new(&env, &lumen_contract_address);
        let caller_balance = client.balance_of(&caller);

        assert!(caller_balance >= amount_to_assign);

        let actual_contract_address = env.current_contract_address();

        client.transfer_from(&actual_contract_address, &caller, &actual_contract_address, &amount_to_assign);

        env.storage()
            .persistent()
            .set(&DataKey::AssignedCreditMap(caller.clone(), address_to_assign.clone()), &amount_to_assign.clone());
    }

    pub fn get_assigned_collateral(env: Env, guarantor_address: Address, beneficiary_address: Address) -> u32
    {
        let assigned_collateral = env.storage()
            .persistent()
            .get(&DataKey::AssignedCreditMap(guarantor_address, beneficiary_address))
            .unwrap_or(0);

        assigned_collateral
    }   

    pub fn simulate_credit_payments(env: Env, usd_to_send: u32) -> u32
    {
        let payment = usd_to_send / 4;
        let interest = payment / 100;
        let paymentWithInterest = payment + interest;

        paymentWithInterest
    }

    pub fn send_amount_using_credit(env: Env, caller: Address, collateral_guarantor_address: Address, usd_to_send: u32, address_to_send: Address, usdc_contract_address: Address)
    {
        caller.require_auth();

        let assigned_collateral = env.storage()
            .persistent()
            .get(&DataKey::AssignedCreditMap(collateral_guarantor_address.clone(), caller.clone()))
            .unwrap_or(0);

        let price: u32 = env.storage().instance().get(&PRICE).unwrap_or(0);
        let value = assigned_collateral * &price;

        assert!(value >= usd_to_send);

        let payment = usd_to_send / 4;
        let interest = payment / 100;
        let paymentWithInterest = payment + interest;
     
        env.storage()
            .persistent()
            .set(&DataKey::PaymentsWithInterests(caller.clone()), &paymentWithInterest.clone());

        let actual_contract_address = env.current_contract_address();

        let client = usdc_contract::Client::new(&env, &usdc_contract_address);
        client.transfer(&actual_contract_address, &address_to_send, &usd_to_send);


        //     // a - totalCollateral  ->   b - value
        //     // c - ?                ->   x - usdcToSend

        //     // x = (b * c) / a  ->   usdcToSend = (value * ?) / totalCollateral
        //                              usdcToSend * totalCollateral = value * ?
        //                              (usdcToSend * totalCollateral) / value = ?


        let usedCollateral = ( usd_to_send * assigned_collateral ) / value;

        // UsedCreditMap(Address, Address),
        let actualUsedCollateral = env.storage()
        .persistent()
        .get(&DataKey::UsedCreditMap(collateral_guarantor_address.clone(), caller.clone()))
        .unwrap_or(0);

        let totalCollateralUsed = actualUsedCollateral + usedCollateral;

        env.storage()
            .persistent()
            .set(&DataKey::UsedCreditMap(collateral_guarantor_address, caller), &totalCollateralUsed.clone());

    }

    pub fn paymentsWithInteresPaids(env: Env, user_address: Address) -> u32
    {
        let mut payments_paid: u32 = 0;

        for i in 1..=4
        {
            let paid = env.storage()
            .persistent()
            .get(&DataKey::PaymentsPaidMap(user_address.clone(), i.clone()))
            .unwrap_or(false);

            if paid == true
            {
                payments_paid += 1;
            }
        }

        payments_paid
    }
}

mod test;