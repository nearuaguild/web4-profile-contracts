use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json};
use near_sdk::collections::LookupMap;
use near_sdk::{
    env, near_bindgen, Balance, Gas, PanicOnDefault, Promise, AccountId, log, PromiseError
};

mod utils;

const NEAR_PER_STORAGE: Balance = 10_000_000_000_000_000_000; // 10e18yⓃ
const CREATE_ACCOUNT_GAS: Gas = Gas(120_000_000_000_000); // 120 TGas
const INITIATE_ACCOUNT_GAS: Gas = Gas(20_000_000_000_000); // 20 TGas
const CALLBACK_GAS: Gas = Gas(20_000_000_000_000); // 20 TGas

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LittleLinkFactory {
    littlelink_contract_by_creator: LookupMap<AccountId, AccountId>,
    code: Option<Vec<u8>>,
    owner_id: AccountId,
    ipfs: Option<String>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PageArgs {
    title: String,
    description: String,
    links: Vec<LinkItem>
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LinkItem {
    text: String,
    path: String,
    r#type: i32 // represent link_type enum
}

#[near_bindgen]
impl LittleLinkFactory {
    #[init]
    pub fn new() -> Self {
        let owner = env::predecessor_account_id();

        Self {
            littlelink_contract_by_creator: LookupMap::new(b"c"),
            owner_id: owner,
            ipfs: None,
            code: None
        }
    }
    
    fn assert_owner(&self) {
        utils::assert_condition(self.owner_id == env::predecessor_account_id(), "This method can be accessed only by the owner");
    }

    pub fn set_code(&mut self) {
        self.assert_owner();

        let code = env::input().expect("No code provided");

        self.code = Some(code.to_vec());
    }

    pub fn set_ipfs(&mut self, ipfs: String) {
        self.assert_owner();

        self.ipfs = Some(ipfs);
    }

    pub fn get_registered_contract_by_creator(&self, creator_id: AccountId) -> Option<AccountId> {
        self.littlelink_contract_by_creator.get(&creator_id)
    }

    fn internal_check_contract_registered(&self, creator_id: &AccountId) -> bool {
        self.littlelink_contract_by_creator.contains_key(&creator_id)
    }

    #[payable]
    pub fn create_web4_little_link_page(&mut self, page_data: PageArgs, pub_key: String) -> Promise {
        let index = env::current_account_id().to_owned().to_string().rfind('.').unwrap();
        let network_slice = &env::current_account_id().to_owned().to_string()[(index + 1)..];
        let network = network_slice.to_owned();

        let creator_id = env::predecessor_account_id();

        assert!(
            !self.internal_check_contract_registered(&creator_id.clone().try_into().unwrap()),
            "LittleLink contract is already registered for you"
        );

        let code = self.code.clone().expect("No code found, owner must provide this");
        let ipfs = self.ipfs.clone().expect("No IPFS gateway found, owner must provide this");

        let minimum_needed = NEAR_PER_STORAGE * code.len() as u128 + 350_000_000_000_000_000_000_000;

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
            "contract_bytes": &code,
        }}).to_string().into_bytes();
        
        let create_account_promise = Promise::new(network.clone().try_into().unwrap()).function_call(
            "create_account_advanced".to_string(),
            create_account_args, minimum_needed, CREATE_ACCOUNT_GAS
        );
        
        let initiate_contract_args = json!({
            "data": page_data,
            "ipfs": &ipfs,
            "owner_id": creator_id.clone().to_owned()
        }).to_string().into_bytes();
        
        let initiate_contract_promise = Promise::new(web4_contract_id.clone().try_into().unwrap()).function_call("init".to_string(), initiate_contract_args, 0, INITIATE_ACCOUNT_GAS);

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
    )-> Option<String> {
        if let Ok(_result) = create_deploy_result {
            log!(format!("Correctly created and deployed to {web4_contract_id}"));
            return Some(web4_contract_id.to_string());
        };

        log!(format!(
            "Error creating {web4_contract_id} - refunding deposit & removing from contracts map"
        ));

        self.littlelink_contract_by_creator.remove(&creator_id);

        Promise::new(creator_id.to_owned()).transfer(attached_deposit);

        return None;
    }
}