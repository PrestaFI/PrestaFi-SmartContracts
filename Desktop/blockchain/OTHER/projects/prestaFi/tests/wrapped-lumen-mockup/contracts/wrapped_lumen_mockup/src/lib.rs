#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, BytesN, Env, IntoVal, Vec, log, symbol_short, Symbol, map};

const COUNTER: Symbol = symbol_short!("COUNTER");
const NAME: Symbol = symbol_short!("NAME");
const SYMBOL: Symbol = symbol_short!("SYMBOL");
const DECIMALS: Symbol = symbol_short!("DECIMALS");
const SUPPLY: Symbol = symbol_short!("SUPPLY");



#[derive(Clone)]
#[contracttype]
pub enum DataKey 
{
    ShareBalances(Address),
    ShareBalance,
    Allowances(Address),
    Allowance,

    BalanceMap(Address),
    AllowanceMap(Address, Address),
}

#[contract]
pub struct UsdcMockup;

#[contractimpl]
impl UsdcMockup 
{
    pub fn mint(env: Env, address_to_mint: Address, amount_to_mint: u32) -> u32
    {
        let mut user_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(address_to_mint.clone()))
            .unwrap_or(0);

        user_balance += amount_to_mint;

        env.storage().persistent().set(&DataKey::BalanceMap(address_to_mint.clone()), &user_balance);

        let mut supply: u32 = env.storage().instance().get(&SUPPLY).unwrap_or(0);
        supply += amount_to_mint;
        env.storage().instance().set(&SUPPLY, &supply);

        supply
    }

    pub fn balance_of(env: Env, address_to_check: Address) -> u32
    {
        let mut user_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(address_to_check.clone()))
            .unwrap_or(0);

        user_balance
    }
    
    pub fn transfer(env: Env, from: Address, to: Address, amount: u32)
    {
        from.require_auth();

        let mut from_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(from.clone()))
            .unwrap_or(0);

        assert!(from_balance >= amount);

        let mut to_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(to.clone()))
            .unwrap_or(0);

        from_balance -= amount;
        to_balance += amount;

        env.storage().persistent().set(&DataKey::BalanceMap(from.clone()), &from_balance);
        env.storage().persistent().set(&DataKey::BalanceMap(to.clone()), &to_balance);
    }

    pub fn set_allowance(env: Env, caller: Address, spender: Address, amount: u32)
    {
        caller.require_auth();

        let caller_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(caller.clone()))
            .unwrap_or(0);

        assert!(caller_balance >= amount);

        env.storage()
            .persistent()
            .set(&DataKey::AllowanceMap(caller.clone(), spender.clone()), &amount.clone());
    }
 
    pub fn allowance(env: Env, owner: Address, spender: Address) -> u32
    {
        env.storage()
            .persistent()
            .get(&DataKey::AllowanceMap(owner, spender))
            .unwrap_or(0)
    }

    pub fn transfer_from(env: Env, caller: Address, from: Address, to: Address, amount: u32)
    {
        caller.require_auth();

        let mut from_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(from.clone()))
            .unwrap_or(0);

        assert!(from_balance >= amount);

        let mut caller_allowance = Self::allowance(env.clone(), from.clone(), caller.clone());

        assert!(caller_allowance >= amount);

        let mut to_balance: u32 = 
        env.storage()
            .persistent()
            .get(&DataKey::BalanceMap(to.clone()))
            .unwrap_or(0);

        from_balance -= amount;
        to_balance += amount;

        env.storage().persistent().set(&DataKey::BalanceMap(from.clone()), &from_balance);
        env.storage().persistent().set(&DataKey::BalanceMap(to.clone()), &to_balance); 

        caller_allowance -= amount;

        env.storage()
            .persistent()
            .set(&DataKey::AllowanceMap(from.clone(), caller.clone()), &caller_allowance.clone());
    }

    pub fn get_total_supply(env: Env) -> u32
    {
        let supply: u32 = env.storage().instance().get(&SUPPLY).unwrap_or(0);

        supply
    }
}

mod test;
