use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json};
use near_sdk::collections::LookupMap;
use near_sdk::{
    env, near_bindgen, Balance, Gas, PanicOnDefault, Promise, AccountId, log, PromiseError
};

const NEAR_PER_STORAGE: Balance = 10_000_000_000_000_000_000; // 10e18yⓃ
const CREATE_ACCOUNT_GAS: Gas = Gas(150_000_000_000_000); // 150 TGas
const INITIATE_ACCOUNT_GAS: Gas = Gas(15_000_000_000_000); // 15 TGas
const CALLBACK_GAS: Gas = Gas(15_000_000_000_000); // 15 TGas

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LittleLinkFactory {
    littlelink_contract_by_creator: LookupMap<AccountId, AccountId>,
    code: Vec<u8>
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PageArgs {
    title: String,
    description: String,
    github: Option<String>,
    twitter: Option<String>,
    medium: Option<String>,
    telegram: Option<String>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Args {
    data: PageArgs
}

#[near_bindgen]
impl LittleLinkFactory {
    #[init]
    pub fn new() -> Self {
        let code = env::input().expect("No code provided");

        Self {
            littlelink_contract_by_creator: LookupMap::new(b"c"),
            code: code.to_vec()
        }
    }

    pub fn get_registered_contract_by_creator(&self, creator_id: AccountId) -> Option<AccountId> {
        self.littlelink_contract_by_creator.get(&creator_id)
    }

    fn internal_check_contract_registered(&self, creator_id: &AccountId) -> bool {
        self.littlelink_contract_by_creator.contains_key(&creator_id)
    }

    #[payable]
    pub fn create_web4_little_link_page(&mut self, args: Args, pub_key: String) -> Promise {
        let index = env::current_account_id().to_owned().to_string().rfind('.').unwrap();
        let network_slice = &env::current_account_id().to_owned().to_string()[(index + 1)..];
        let network = network_slice.to_owned();

        let creator_id = env::predecessor_account_id();

        assert!(
            !self.internal_check_contract_registered(&creator_id.clone().try_into().unwrap()),
            "LittleLink contract is already registered for you"
        );

        let minimum_needed = NEAR_PER_STORAGE * self.code.len() as u128;

        assert!(
            env::attached_deposit() >= minimum_needed,
            "Attach at least {minimum_needed} yⓃ"
        );

        let random_seed: Vec<u8> = env::random_seed().to_owned();

        let random: String = hex::encode::<Vec<u8>>(random_seed);

        let slice = &random[..24];

        let web4_contract_id = format!("{slice}.{network}");

        self.littlelink_contract_by_creator.insert(&creator_id.clone(), &web4_contract_id.clone().try_into().unwrap());
        
        let create_account_args = json!({"new_account_id": web4_contract_id.clone(), "options": {
            "full_access_keys": [&pub_key.clone()],
            "contract_bytes": &self.code,
        }}).to_string().into_bytes();
        
        let create_account_promise = Promise::new(network.clone().try_into().unwrap()).function_call(
            "create_account_advanced".to_string(),
            create_account_args, minimum_needed + 100_000_000_000_000_000_000_000, CREATE_ACCOUNT_GAS
        );
        
        let initiate_contract_args = json!({
            "data": args.data,
            "ipfs": "ipfs://bafybeihiw6mwe63nvy6it7sialrr3fgx57jqnfdqlpxg7do6fytbxnzmna",
            "owner_id": creator_id.clone().to_owned()
        }).to_string().into_bytes();
        
        let initiate_contract_promise = Promise::new(web4_contract_id.clone().try_into().unwrap()).function_call("new".to_string(), initiate_contract_args, 0, INITIATE_ACCOUNT_GAS);

        create_account_promise.then(initiate_contract_promise).then(
                Self::ext(env::current_account_id()).with_static_gas(CALLBACK_GAS).create_web4_little_link_page_callback(
                    creator_id.clone().try_into().unwrap(),
                    web4_contract_id.clone().try_into().unwrap(),
                    env::attached_deposit()
                ),
            )
    }

    #[private]
    pub fn create_web4_little_link_page_callback(
        &mut self,
        creator_id: AccountId, 
        web4_contract_id: AccountId, 
        attached_deposit: Balance, 
        #[callback_result] create_deploy_result: Result<(), PromiseError>
    )-> bool {
        if let Ok(_result) = create_deploy_result {
            log!(format!("Correctly created and deployed to {web4_contract_id}"));
            return true;
        };

        log!(format!(
            "Error creating {web4_contract_id}, refunding deposit & removing from contracts map"
        ));

        self.littlelink_contract_by_creator.remove(&creator_id);

        Promise::new(creator_id.to_owned()).transfer(attached_deposit);

        false
    }
}