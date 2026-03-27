#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Address, Symbol, Vec, Map
};

#[derive(Clone)]
#[contracttype]
pub struct Property {
    pub owner: Address,
    pub total_tokens: u32,
    pub available_tokens: u32,
    pub price_per_token: i128,
}

#[contract]
pub struct RealEstateContract;

#[contractimpl]
impl RealEstateContract {

    // Initialize property
    pub fn init_property(
        env: Env,
        owner: Address,
        total_tokens: u32,
        price_per_token: i128,
    ) {
        owner.require_auth();

        let property = Property {
            owner: owner.clone(),
            total_tokens,
            available_tokens: total_tokens,
            price_per_token,
        };

        env.storage().instance().set(&Symbol::short("PROP"), &property);
    }

    // Buy tokens (fractional ownership)
    pub fn buy_tokens(env: Env, buyer: Address, amount: u32) {
        buyer.require_auth();

        let mut property: Property = env
            .storage()
            .instance()
            .get(&Symbol::short("PROP"))
            .unwrap();

        if property.available_tokens < amount {
            panic!("Not enough tokens available");
        }

        property.available_tokens -= amount;

        // Store ownership mapping
        let mut ownership: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&Symbol::short("OWN"))
            .unwrap_or(Map::new(&env));

        let current = ownership.get(buyer.clone()).unwrap_or(0);
        ownership.set(buyer.clone(), current + amount);

        env.storage().instance().set(&Symbol::short("OWN"), &ownership);
        env.storage().instance().set(&Symbol::short("PROP"), &property);
    }

    // Check ownership
    pub fn get_tokens(env: Env, user: Address) -> u32 {
        let ownership: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&Symbol::short("OWN"))
            .unwrap_or(Map::new(&env));

        ownership.get(user).unwrap_or(0)
    }

    // Property details
    pub fn get_property(env: Env) -> Property {
        env.storage()
            .instance()
            .get(&Symbol::short("PROP"))
            .unwrap()
    }
}