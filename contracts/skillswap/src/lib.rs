#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillProfile {
    pub user: Address,
    pub teach: String,
    pub learn: String,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Profile(Address),
}

#[contractimpl]
impl Contract {
    pub fn create_profile(
        env: Env,
        user: Address,
        teach: String,
        learn: String,
    ) -> SkillProfile {
        user.require_auth();

        let key = DataKey::Profile(user.clone());

        if env.storage().persistent().has(&key) {
            panic!("profile already exists");
        }

        let profile = SkillProfile {
            user,
            teach,
            learn,
            active: true,
        };

        env.storage().persistent().set(&key, &profile);

        profile
    }

    pub fn update_profile(
        env: Env,
        user: Address,
        teach: String,
        learn: String,
    ) -> SkillProfile {
        user.require_auth();

        let key = DataKey::Profile(user.clone());

        if !env.storage().persistent().has(&key) {
            panic!("profile not found");
        }

        let profile = SkillProfile {
            user,
            teach,
            learn,
            active: true,
        };

        env.storage().persistent().set(&key, &profile);

        profile
    }

    pub fn get_profile(env: Env, user: Address) -> SkillProfile {
        let key = DataKey::Profile(user);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("profile not found"))
    }

    pub fn has_profile(env: Env, user: Address) -> bool {
        let key = DataKey::Profile(user);

        match env.storage().persistent().get::<DataKey, SkillProfile>(&key) {
            Some(profile) => profile.active,
            None => false,
        }
    }

    pub fn delete_profile(env: Env, user: Address) -> bool {
        user.require_auth();

        let key = DataKey::Profile(user.clone());

        let mut profile: SkillProfile = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("profile not found"));

        profile.active = false;

        env.storage().persistent().set(&key, &profile);

        true
    }
}

mod test;