
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId,Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
 

near_sdk::setup_alloc!();
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;
#[derive(BorshDeserialize, BorshSerialize )]
pub struct OldContract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_token_on_sale: u64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_token_on_sale: u64,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meta {
    token_id : String,
    owner_id : String,
    title : String,
    description : String,
    media : String,
    culture : String,
    country : String,
    creator : String,
    price : String,
    on_sale: bool, // sale status
    on_auction: bool, //auction status
    adressbidder: String,
    highestbidder: String,
    lowestbidder: String,
    expires_at: String,
    starts_at: String,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Extras {
    culture : String,
    country : String,
    creator : String,
    price : String,
    on_sale: bool, // sale status
    on_auction: bool, //auction status
    adressbidder: String,
    highestbidder: String,
    lowestbidder: String,
    expires_at: String,
    starts_at: String,
}
impl Default for Contract {
    
    fn default( ) -> Self {
      
     let meta =NFTContractMetadata {
         spec: NFT_METADATA_SPEC.to_string(),
         name: "Nativo NFT".to_string(),
         symbol: "NTV".to_string(),
         icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
         base_uri: None,
         reference: None,
         reference_hash: None,
     };
     Self {
     
         tokens:NonFungibleToken::new(
             StorageKey::NonFungibleToken,
             env::signer_account_id().try_into().unwrap(),
             Some(StorageKey::TokenMetadata),
             Some(StorageKey::Enumeration),
             Some(StorageKey::Approval),
         ) ,
         metadata: LazyOption::new(StorageKey::Metadata, Some(&meta)),
         n_token_on_sale:0 ,
         
     }   }
}
 
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    // Esta función incializa el contrato con los valores especificados en la metadata

    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Nativo NFT".to_string(),
                symbol: "NTV".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_token_on_sale: 0,
        }
    }

    fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
        let metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .unwrap()
            .get(&token_id);
        let approved_account_ids = Some(
            self.tokens
                .approvals_by_id
                .as_ref()
                .unwrap()
                .get(&token_id)
                .unwrap_or_default(),
        );

        Token {
            token_id,
            owner_id,
            metadata,
            approved_account_ids,
        }
    }


    //all methods created 
      /**
     * permite minar nuevos tokens
     * @param token_owner_id {ValidAccountId} a quien le va a pertenecer el token
     * @param tokenMetadata {TokenMetadata} los metadatos
     */
    #[payable]
    pub fn minar(&mut self,token_owner_id: ValidAccountId,token_metadata: TokenMetadata,) -> Token {
        self.n_token_on_sale += 1;
        self.tokens.mint(
            self.tokens.owner_by_id.len().to_string(),
            token_owner_id,
            Some(token_metadata),
        )
    }

    // #[payable]
    // pub fn comprar_nft(&mut self, token_id: TokenId) -> TokenMetadata {
    //     //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
    //     //let token_id_u64 = token_id.parse::<u64>().unwrap();

    //     assert_eq!(
    //         token_id.trim().parse::<u64>().unwrap() < self.tokens.owner_by_id.len(),
    //         true,
    //         "ese token no existe "
    //     );
    //     //obtener los metadatos de ese token
    //     let mut metadata = self
    //         .tokens
    //         .token_metadata_by_id
    //         .as_ref()
    //         .and_then(|by_id| by_id.get(&token_id))
    //         .unwrap();

    //     // si no cuenta con los fondos hacemos rollback
    //     let amount = env::attached_deposit();
    //     assert_eq!(
    //         metadata.price.as_ref().unwrap().parse::<u128>().unwrap(),
    //         amount,
    //         "fondos insuficientes"
    //     );
    //     assert_eq!(
    //         metadata.on_sale.as_ref().unwrap(),
    //         &true, 
    //         "no esta a la venta"
    //     );

    //     //revisar que este a la venta
    //     //obtener el dueño del token
    //     let token_owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
    //     //obtener el creador del token
    //     let creator_id = metadata.creator.as_ref().unwrap();
    //     //obtener el comprador del token
    //     let buyer_id = &env::signer_account_id();
    //      //obtener la regalia,la comision de Nativo y el pagoa al autor del token
    //      let mut  res:f64=0.0;
    //      let mut  roy:f64=0.0;
    //      let mut  gains:f64=0.0;
    //      let mut  pay:f64=0.0;
    //      roy = amount as f64 *0.10;
    //      gains=amount as f64 *0.03;
    //      pay=amount as f64 *0.87;
          
    //     let my_string = metadata.price.as_ref().unwrap().to_string();  // `parse()` works with `&str` and `String`!
    //     let price_meta = my_string.parse::<u128>().unwrap();
    //     let regal = amount-price_meta ;

       
    //     // el dueñp no puede comprar su propio token
    //     assert_eq!(buyer_id == &token_owner_id, false, "eres el dueño del token ");
    //     //cambiar la metadata
    //     metadata.on_sale = Some(false);
    //     //remplazamos la metadata
    //     self.tokens
    //         .token_metadata_by_id
    //         .as_mut()
    //         .and_then(|by_id| by_id.insert(&token_id, &metadata));
    //     //transferir los nears
    //     //TODO: entender como consultar si la transferencia fue exitosa

    //     /*
    //     let promise = Promise::new(owner_id.clone())
    //         .transfer(amount)
    //         .function_call("tx_status_callback".into(), vec![], 0, 0);
    //     */
    //     Promise::new(token_owner_id.clone()).transfer(pay as  u128);
    
    //     //TODO: transferir la regalia del token
    //     Promise::new(creator_id.clone()).transfer(gains as u128);
    //     //TODO: transferir la regalia del token
    //     Promise::new(env::predecessor_account_id().clone()).transfer(roy as u128);
    //     //transferir el nft
    //     self.tokens
    //         .internal_transfer_unguarded(&token_id, &token_owner_id, buyer_id);

    //     //cambiar el numero de nfts disponibles
    //     self.nTokenOnSale -= 1;
    //     //retornar la metadata
    //     metadata
    // }

    pub fn storage_byte_cost() -> Balance {
        env::storage_byte_cost()
    }

    pub fn get_on_sale_toks(&self) -> u64 {
        self.n_token_on_sale
    }

    pub fn update_token(&mut self, token_id: TokenId, extra: String) -> TokenMetadata {
        //assert!(!env::state_exists(), "Already initialized");
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        metadata.extra = Some(extra);
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));
        metadata
    }

    pub fn get_token(&self, token_id: TokenId, owner_id: AccountId) -> Meta {
        
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();
        let token = Meta {
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
            title : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            media : metadata.media.as_ref().unwrap().to_string(),
            culture : extradatajson.culture,
            country : extradatajson.country,
            creator : extradatajson.creator,
            price : extradatajson.price,
            on_sale: extradatajson.on_sale, // sale status
            on_auction: extradatajson.on_auction, //auction status
            adressbidder: extradatajson.adressbidder,
            highestbidder: extradatajson.highestbidder,
            lowestbidder:extradatajson.lowestbidder,
            expires_at: extradatajson.expires_at,
            starts_at: extradatajson.starts_at,
        };
        token
    }
 
    pub fn obtener_pagina_v2(&self, from_index: usize, limit: u64) -> Vec<Meta> {
        // no estoy segyri de como convierte  de U128 a u128
        let start_index: u128 = Some(from_index).map(|v| v as u128).unwrap_or_default();
        let limit = Some(limit).map(|v| v as usize).unwrap_or(usize::MAX);
        let inicioPag = start_index as usize * limit;

        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        let mut counter: usize = 0;
        self.tokens
            .owner_by_id
            .iter()
            .take(limit)
            .map(|(token_id, owner_id)| self.get_token(token_id, owner_id))
            .collect()
    }
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        Self {
           
            tokens:old_state.tokens,
            metadata: old_state.metadata,
            n_token_on_sale: old_state.n_token_on_sale,
        }
    }
   


}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}


