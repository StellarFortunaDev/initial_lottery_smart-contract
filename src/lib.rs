#![no_std]
use soroban_sdk::{
    contract, contractimpl, Address, Env, Vec, 
    IntoVal, Val
};

#[derive(Clone)]
pub enum DataKey {
    Players,
    Owner,
    WinningNumber,
    Status,
}

impl IntoVal<Env, Val> for DataKey {
    fn into_val(&self, env: &Env) -> Val {
        let key = match self {
            DataKey::Players => 1u32,
            DataKey::Owner => 2u32,
            DataKey::WinningNumber => 3u32,
            DataKey::Status => 4u32,
        };
        key.into_val(env)
    }
}

#[contract]
pub struct Lottery;

#[contractimpl]
impl Lottery {
    pub fn init(env: Env, owner: Address) {
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Status, &false);
        let empty_vec: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Players, &empty_vec);
    }

    pub fn join(env: Env, player: Address) {
        let status: bool = env.storage().instance().get(&DataKey::Status).unwrap();
        if status {
            panic!("Lottery is closed");
        }

        let mut players: Vec<Address> = env.storage().instance().get(&DataKey::Players).unwrap();
        if !players.contains(&player) {
            players.push_back(player);
            env.storage().instance().set(&DataKey::Players, &players);
        }
    }

    pub fn draw(env: Env, owner: Address) -> Address {
        let stored_owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        if owner != stored_owner {
            panic!("Only owner can draw");
        }

        let players: Vec<Address> = env.storage().instance().get(&DataKey::Players).unwrap();
        if players.is_empty() {
            panic!("No players in lottery");
        }

        env.storage().instance().set(&DataKey::Status, &true);
        
        let random_index: u64 = env.prng().gen_range(0u64..players.len() as u64);
        players.get(random_index as u32).unwrap()
    }

    pub fn get_players(env: Env) -> Vec<Address> {
        env.storage().instance().get(&DataKey::Players).unwrap()
    }

    pub fn get_status(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Status).unwrap()
    }
}
