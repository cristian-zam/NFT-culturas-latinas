
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

// use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
///home/josecanales/Github/Nativo-NFT/blockchain/rust-contract/near-contract-standards-3.2.0/src/non_fungible_token/core/core_impl.rs
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use near_sdk::collections::LazyOption;
//use std::time::{Duration, Instant};
//use std::thread::sleep;


use near_sdk::json_types::{ValidAccountId,Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use serde_json::json;
use std::convert::TryInto;

near_sdk::setup_alloc!();
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;


#[derive(BorshDeserialize, BorshSerialize )]
pub struct OldContract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
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

lazy_static! {
    static ref USER_TOKEN_HASHMAP: Mutex<HashMap<String, HashMap<String,String>>> = Mutex::new(HashMap::new());
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
         n_total_tokens: 0,
         n_token_on_sale: 0,
         n_token_on_auction: 0,
         
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
            n_total_tokens: 0,
            n_token_on_sale: 0,
            n_token_on_auction: 0,
        }
    }
    
    pub fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
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
        let Contractaccount: AccountId = "nativodeploy.testnet".parse().unwrap();
        let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap();
       // log!("token_owner_id {} ,contr {} ",&token_owner_id ,&account.to_string());
       let token_id: TokenId =self.n_total_tokens.to_string();
       

       let mined= self.tokens.mint(
            token_id.clone(),
            account,   // token_owner_id,//cambiar por la direccion del contrato this.address
            Some(token_metadata),
        ); //Retorna    x.token_id
            //una vez minada hacer un tranfer del contract al owner
             //transferir el nft
              
              log!("tokenid {}",&token_id);
          
        self.tokens
        .internal_transfer_unguarded(&token_id, &Contractaccount.to_string(), &token_owner_id.to_string());
        log!("tranfer done");
        self.n_total_tokens  +=1;
        self.n_token_on_sale += 1;
        let mut originaltoken = Contract::enum_get_token( &self,self.tokens.owner_by_id.get(&token_id.to_string()).unwrap(),token_id.clone());
        originaltoken
    }

    #[payable]
    pub fn comprar_nft(&mut self, token_id: TokenId) -> TokenMetadata {
        let Contractaccount: AccountId = "nativodeploy.testnet".parse().unwrap();

        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        //let token_id_u64 = token_id.parse::<u64>().unwrap();
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
            /* {
                title: 'Mr Burrito',
                description: 'This is a burrito',
                media: 'imagenimagenimagenimagenimagenim',
                media_hash: null,
                copies: null,
                issued_at: null,
                expires_at: null,
                starts_at: null,
                updated_at: null,
                extra: "{'culture':'Burriroca','country':'BurritoLand','creator':'dev-1636751893359-19496702378959','price':'20','on_sale':true,'on_auction':false,'adressbidder':'accountbidder','highestbidder':'notienealtos','lowestbidder':'notienebajos','expires_at':'noexpira','starts_at':'noinicia'}",
                reference: null,
                reference_hash: null
              } */
            let owner_id = self.tokens.owner_by_id.get(&token_id);
            let owner_value = owner_id.as_deref().unwrap_or("default string");
            let mut metadataextra = Contract::get_token(self, token_id.clone());
        /*     {
                token_id: '1',
                owner_id: 'dev-1636751893359-19496702378959',
                title: 'Mr Burrito',
                description: 'This is a burrito',
                media: 'imagenimagenimagenimagenimagenim',
                culture: 'Burriroca',
                country: 'BurritoLand',
                creator: 'dev-1636751893359-19496702378959',
                price: '20',
                on_sale: true,
                on_auction: false,
                adressbidder: 'accountbidder',
                highestbidder: 'notienealtos',
                lowestbidder: 'notienebajos',
                expires_at: 'noexpira',
                starts_at: 'noinicia'
            } */
            
            // si no cuenta con los fondos hacemos rollback
        let amount = env::attached_deposit();
        assert_eq!(
            metadataextra.price.parse::<u128>().unwrap(),
            amount,
            "Cantidad incorrecta,verifica el costo exacto!"
        );
        assert_eq!(
            metadataextra.on_sale,
            true, 
            "no esta a la venta"
        );
        
        //revisar que este a la venta
        //obtener el dueño del token
        //let token_owner_id = self.tokens.owner_by_id.get(token_id).unwrap();
        //obtener el creador del token
        let creator_id = metadataextra.creator;
        //obtener el comprador del token
        let buyer_id = &env::signer_account_id();

        // el dueñp no puede comprar su propio token
        assert_eq!(buyer_id == &owner_value, false, "eres el dueño del token ");
         //obtener la regalia,la comision de Nativo y el pagoa al autor del token
         let mut  res:f64=0.0;
         let mut  roy:f64=0.0;
         let mut  gains:f64=0.0;
         let mut  pay:f64=0.0;
         roy = amount as f64 *0.10;
         gains=amount as f64 *0.03;
         pay=amount as f64 *0.87;
          
        /* let my_string = metadataextra.price.as_ref().unwrap().to_string();  // `parse()` works with `&str` and `String`!
        let price_meta = my_string.parse::<u128>().unwrap();
        let regal = amount-price_meta ; */

        log!("{}",&originaltoken.extra.as_ref().unwrap().to_string());
       
        //cambiar la metadata
        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
        //Se modifica el json
        extradatajson.on_sale = false;
        //  extradatajson: Extras = serde_json::to(&newextradata).unwrap();    
        log!("{}",&extradatajson.on_sale.to_string());
        // se convierte el Json a un String plano
        let mut extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
        log!("{}",&finalextrajson.to_string());
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //transferir los nears
        //TODO: entender como consultar si la transferencia fue exitosa

        /*
        let promise = Promise::new(owner_id.clone())
            .transfer(amount)
            .function_call("tx_status_callback".into(), vec![], 0, 0);
        */
        Promise::new(owner_value.clone().to_string()).transfer(pay as  u128);
        //TODO: transferir la regalia del token
        Promise::new(creator_id.clone()).transfer(roy as u128);
        //TODO: transferir la regalia del token
        Promise::new(Contractaccount.clone()).transfer(gains as u128);
        //transferir el nft
        self.tokens
            .internal_transfer_unguarded(&token_id, &owner_value.to_string(), buyer_id);

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale -= 1;
        //retornar la metadata
        originaltoken
    }

    // pub fn get_tokens_by_onwer(self,account_id: String, from_index:String,limit:String){
    //     self.tokens.owner_by_id.tokens_per_owner.get(account_id);
    // }

    
    pub fn revender(&mut self, token_id: TokenId, price: String) -> TokenMetadata {
        //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //comprobar que el revendedor sea el owner
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(
            env::signer_account_id() == owner_id,
            true,
            "no eres el dueño del token "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
        //Se modifica el json
        assert_eq!(
            extradatajson.on_auction==true,
            false,
            "Lo sentimos,este token se encuentra en subasta y aun no termina!"
        );
        if price.trim().parse::<u128>().unwrap() > 0 {
            extradatajson.price =  price ;
        }
        extradatajson.on_sale =  true ;
        //  extradatajson: Extras = serde_json::to(&newextradata).unwrap();    
        log!("{}",&extradatajson.on_sale.to_string());
        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
        log!("{}",&finalextrajson.to_string());
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        //retornar la metadata
        originaltoken
    }

    pub fn subastar_nft(&mut self, token_id: TokenId,lowestbidder:String,starts_at:String,expires_at:String){
     
      
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
           //1.- Verificar que seas el owner del token 
            assert_eq!(token_owner_id != Some(env::signer_account_id().to_string()), false, "no eres el dueño del token ");
           
  
              //cambiar la metadata
                  
                  //se  reemplaza los ' por \" en un string plano                "'", "\""
                  let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
                  //el string plano se convierte a JSon
                  let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
                  //Se modifica el json
                  extradatajson.on_sale = false;
                  extradatajson.on_auction = true;
                  extradatajson.lowestbidder=lowestbidder.clone();
                  extradatajson.price=lowestbidder;
                  extradatajson.highestbidder=0.to_string();
                  extradatajson.expires_at=expires_at;
                  extradatajson.starts_at=starts_at; 
                  // se convierte el Json a un String plano
                  let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
                  let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
                 
                  originaltoken.extra = Some(finalextrajson);
                  //remplazamos la metadata
                  self.tokens
                      .token_metadata_by_id
                      .as_mut()
                      .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
         
                //cambiar el numero de nfts disponibles
                  
                  self.n_token_on_auction+=1;
      }
    
      #[payable]
      pub fn ofertar_subasta(&mut self, token_id: TokenId  ){
        let Contractaccount: AccountId = "nativodeploy.testnet".parse().unwrap();
        let mut amountsended=env::attached_deposit();
        
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
           //1.- Verificar que no seas el owner del token
            assert_eq!(token_owner_id == Some(env::signer_account_id().to_string()), false, "Eres el dueño del token ");
              //cambiar la metadata
                  //se  reemplaza los ' por \" en un string plano                "'", "\""
                  let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
                  //el string plano se convierte a JSon
                  let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
                  //Se modifica el json
                   //Validar sino ha finalizado  
                let expires_at_token=   extradatajson.expires_at.parse::<i64>().unwrap();
                //let local: DateTime = Local::now();
                
              //   let now = Instant::now();
               
            //   //  let noww= chrono::offset::Utc::now().timestamp() ;
                    log!("token date : {},today: {}",(expires_at_token as u64)*1000000,env::block_timestamp());  
             assert_eq!((expires_at_token as u64)*1000000
                        < env::block_timestamp(),
                           false,// self.finalizar_subasta(token_id.clone())
                             "la subasta ya terminó" );  
                             log!("low:{},high:{},send:{}", extradatajson.lowestbidder.parse::<u128>().unwrap(),extradatajson.highestbidder.parse::<u128>().unwrap(),amountsended);
 assert_eq!( extradatajson.lowestbidder.parse::<u128>().unwrap() > amountsended, false, "la cantidad enviada es menos que el minimo");
 assert_eq!( extradatajson.highestbidder.parse::<u128>().unwrap() >= amountsended, false, "la cantidad enviada es menor o igual que la ultima puja");
                                 
                  if extradatajson.lowestbidder.parse::<u128>().unwrap() <= amountsended
                         && extradatajson.highestbidder.parse::<u128>().unwrap() < amountsended {
                    if extradatajson.highestbidder.parse::<u128>().unwrap() > 0 {
                              //regresar el bid al singer anterior
                                Promise::new(extradatajson.adressbidder.clone().to_string()).transfer(extradatajson.highestbidder.parse::<u128>().unwrap());
                        }
                            //actualizar el nuevo  signer y bid
                                extradatajson.highestbidder =amountsended.to_string();
                                extradatajson.adressbidder=env::signer_account_id().to_string();
                                Promise::new(Contractaccount.clone().to_string()).transfer( amountsended.clone() );

                                // se convierte el Json a un String plano
                        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
                        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
                        originaltoken.extra = Some(finalextrajson);
                        //remplazamos la metadata
                        self.tokens
                            .token_metadata_by_id
                            .as_mut()
                            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
                        //cambiar el numero de nfts disponibles
                        // List<token ,List<signer,bid>>  lists de subasta -> lista de offertas
                        let mut bidding_info = HashMap::new();
                        let mut _map = USER_TOKEN_HASHMAP.lock().unwrap();
                        bidding_info.insert(env::signer_account_id().to_string(),amountsended.to_string());
                        _map.insert(token_id,bidding_info);
                  }
                  
      }

      pub fn finalizar_subasta(&mut self, token_id: TokenId) -> bool {
        let Contractaccount: AccountId = "nativodeploy.testnet".parse().unwrap();
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
            //se  reemplaza los ' por \" en un string plano                "'", "\""
            let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
            //el string plano se convierte a JSon
            let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
            //Validar si ha finalizado
           
          /*   assert_eq!( extradatajson.expires_at.parse::<i64>().unwrap()
            <
                 chrono::offset::Utc::now().timestamp()  , false,"la subasta aun no termina" );
 */
            let amount =extradatajson.highestbidder.parse::<f64>().unwrap();
            //si ya termino la subasta
            //pagamos las ganancias al al onwer anterior,ganancias al contrato y  regalias al creator
                     //obtener la regalia,la comision de Nativo y el pagoa al autor del token
                        let mut  res:f64=0.0;
                        let mut  roy:f64=0.0;
                        let mut  gains:f64=0.0;
                        let mut  pay:f64=0.0;
                        roy  = amount  *0.10;
                        gains= amount  *0.03;
                        pay  = amount  *0.87;
                    //transferir los nears
                    Promise::new( token_owner_id.as_ref().unwrap().to_string() ).transfer(pay as  u128);
                    //TODO: transferir la regalia del token
                    Promise::new(extradatajson.creator.clone()).transfer(roy as u128);
                    //TODO: transferir la regalia del token
                    Promise::new(Contractaccount.clone()).transfer(gains as u128);
                        //cambiamos el on sale/on auction a false
                        if(extradatajson.highestbidder.parse::<u128>().unwrap() == 0 as u128){
                            extradatajson.adressbidder=token_owner_id.as_ref().unwrap().to_string();
                        }
                        extradatajson.on_sale=false;
                        extradatajson.on_auction=false;
                        // se convierte el Json a un String plano
                    let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
                    let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
                    originaltoken.extra = Some(finalextrajson);
                    //remplazamos la metadata
                    self.tokens
                        .token_metadata_by_id
                        .as_mut()
                        .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
            //el costo/subastaminima no se modifica hasta que lo ponga a la venta/subasta
            //lo transferimos al nuevo onwer
            //
            self.tokens
            .internal_transfer_unguarded(&token_id, &token_owner_id.as_ref().unwrap().to_string(), &extradatajson.adressbidder.to_string());
            self.n_token_on_auction-=1;
        return false ;
    }
    #[payable]
    pub fn extraer_token(&mut self, token_id: TokenId){
         

        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let mut  originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
         /*    assert_eq!(
                env::signer_account_id() != token_owner_id.as_ref().unwrap().to_string(),
                false,
                "no eres el dueño del token "
            );  */
            let Contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let msj2: String = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let apro: Option<u64> = Some(apro);
             //   self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
            self.tokens.nft_transfer_call(account, token_id, None,msj, "".to_string());
           log!("transfer done");
    }
    #[payable]
    pub fn aproved_token(&mut self, token_id: TokenId){
        let token_owner_id = self.tokens.owner_by_id.get(&token_id);    
        let Contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            let apro: Option<u64> = Some(0);
            self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
    }
      /// 
      pub fn quitar_del_market_place(&mut self, token_id: TokenId) -> TokenMetadata {
        //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() < self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );

        //comprobar que el revendedor sea el owner
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(
            env::signer_account_id() == owner_id,
            true,
            "no eres el dueño del token "
        );



         //obtener los metadatos de ese token
         let mut originaltoken = self
         .tokens
         .token_metadata_by_id.as_ref()
         .and_then(|by_id| by_id.get(&token_id))
         .unwrap();
      
     //se  reemplaza los ' por \" en un string plano                "'", "\""
     let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
     //el string plano se convierte a JSon
     let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
     //Se modifica el json
     
     
     extradatajson.on_sale =  false ;
 
     // se convierte el Json a un String plano
     let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
     let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
    
     originaltoken.extra = Some(finalextrajson);
     //remplazamos la metadata
     self.tokens
         .token_metadata_by_id
         .as_mut()
         .and_then(|by_id| by_id.insert(&token_id, &originaltoken));


        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        //retornar la metadata
        originaltoken
    }


    pub fn storage_byte_cost() -> Balance {
        env::storage_byte_cost()
    }

    pub fn get_on_sale_toks(&self) -> u64 {
        self.n_token_on_sale
    }
    pub fn get_on_auction_toks(&self) -> u64 {
        self.n_token_on_auction
    }

    pub fn get_on_auction_toksV2(&self) -> u64 {
        let mut _realonsale =0;

        let limit = self.get_on_total_toks();
        log!("tokans total {}",&limit);
        for x in 0..(limit-1) { 
                          
                let mut token =self.get_token(x.to_string().clone());
                if token.on_auction{       
                    _realonsale += 1;
                }
                if x== (limit-1) {
                    break;
                }
       }
        _realonsale
    }
    pub fn get_on_total_toks(&self) -> u64 {
        self.n_total_tokens
    }

     /* fn update_token(&mut self, token_id: TokenId, extra: String) -> TokenMetadata {
        //assert!(!env::state_exists(), "Already initialized");
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        //assert_eq!(owner_id!= env::signer_account_id() && owner != ,false,"");
        metadata.extra = Some(extra);
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));
        metadata
    } */

    pub fn get_token(&self, token_id: TokenId) -> Meta {
        
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
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
    
 
    pub fn obtener_pagina_v3(&self, from_index: u64, limit: u64) -> Vec<Meta>  {
        // no estoy segyri de como convierte  de U128 a u128
      /*   let start_index: u128 = Some(from_index).map(|v| v as u128).unwrap_or_default();
        let limit = Some(limit).map(|v| v as usize).unwrap_or(usize::MAX);
        let inicioPag = start_index as usize * limit; */

        let mut vectMEta = vec![];
        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        //let mut counter: usize = from_index;
        log!("  from  -> : {},to -> {}",&from_index,&limit);
        let mut _tokfound =from_index;  //0
        let totoal =  self.get_on_total_toks()-1;
                log!(" {}" ,&totoal);
            for x in from_index..limit { 
                
                if  x>= totoal.clone() {
                    break;
                }
                    let mut token =self.get_token(x.to_string().clone());
                /*     log!("  x -> : {},token {}",&x,&token.token_id );
                  */   log!("  token -> : {:?}",token.token_id.clone() );
                 
                    if token.on_sale{
                        
                    vectMEta.push(token  );
                    _tokfound+=1;
                    
                }
                if x== limit {
                    break;
                }
                
                //if( x == limit ){break; }           
            }
            if vectMEta.len()< 12 && limit < totoal {
                let  newfrom =limit; //12
                let  newlimit = limit+30; //24
               
                for y in newfrom..newlimit { 
                 let mut token =self.get_token(y.to_string().clone());
                 log!("  2da vuetla token -> : {:?}",&y );
                     if token.on_sale{
                         
                        vectMEta.push(token  );
                        _tokfound+=1;
                        
                    }
                     if y== newlimit {
                        break;
                    }

                }
            }
           
            vectMEta
         
    }
    pub fn obtener_pagina_v3_auction(&self, from_index: u64, limit: u64) -> Vec<Meta>  {
        let mut vectMEta = vec![];
        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        //let mut counter: usize = from_index;
        log!("  from  -> : {},to -> {}",&from_index,&limit);
        let mut _tokfound =from_index;  //0
        let totoal =  self.get_on_total_toks()-1;
                log!(" {}" ,&totoal);
            for x in from_index..limit { 
                if x== limit {
                    break;
                }
                if  x>= totoal.clone() {
                    break;
                }
                    let mut token =self.get_token(x.to_string().clone());
                /*     log!("  x -> : {},token {}",&x,&token.token_id );
                  */   log!("  token -> : {:?}",token.token_id.clone() );
                 
                    if token.on_auction{
                        
                    vectMEta.push(token  );
                    _tokfound+=1;
                    
                }
               
                
                //if( x == limit ){break; }           
            }
            if vectMEta.len()< 12 && limit < totoal {
                let  newfrom =limit; //12
                let  newlimit = limit+30; //24
               
                for y in newfrom..newlimit { 
                 let mut token =self.get_token(y.to_string().clone());
                 log!("  2da vuetla token -> : {:?}",&y );
                     if token.on_auction{
                         
                        vectMEta.push(token  );
                        _tokfound+=1;
                        
                    }
                     if y== newlimit {
                        break;
                    }

                }
            }
           
            vectMEta
         
    }
    pub fn obtener_pagina_v3_by_owner(&self,account: ValidAccountId) -> Vec<Meta>  {
        let mut vectMEta = vec![];
      
       
        let totoal =  self.get_on_total_toks();
        log!("  brakes here  -> : {}",&totoal );
            for x in 0..totoal { 
           
                if x == totoal { break; } 
              
                 let mut token =self.get_token(x.to_string().clone());
            /*    
                log!("  token -> : {:?}",token.token_id.clone() );
             */    
                if token.owner_id==account.to_string(){
                        vectMEta.push(token  );
                }
           }
            vectMEta
         
    }
    pub fn obtener_pagina_v4(&self, from_index: u64, limit: u64) -> Vec<Meta>  {
      
        let mut vectMEta = vec![];
        assert_ne!(limit, 0, "Cannot provide limit of 0.");  //0 //30 -> 30:45
        //let mut counter: usize = from_index;                //46 //30 -> 30:89                      
        log!("  from  -> : {},to -> {}",&from_index,&limit);
        let mut _tokfound =0;  //0
        let totoal =  self.get_on_total_toks()-1;
                log!(" {}" ,&totoal);
            for x in from_index..totoal { 
                
                if  x>= totoal.clone() { //0 ->150   //29-> 150
                    break;
                }
                    let mut token =self.get_token(x.to_string().clone());
               
                 
                    if token.on_sale{
                        
                    vectMEta.push(token  );
                    _tokfound+=1;
                    
                     }
                if _tokfound== limit {  //1 <30  //30-30
                    break;
                }
                
                          
            }
            /* if vectMEta.len()< 12 && limit < totoal {
                let  newfrom =limit; //12
                let  newlimit = limit+30; //24
               
                for y in newfrom..newlimit { 
                 let mut token =self.get_token(y.to_string().clone());
                 log!("  2da vuetla token -> : {:?}",&y );
                     if token.on_sale{
                         
                        vectMEta.push(token  );
                        _tokfound+=1;
                        
                    }
                     if y== newlimit {
                        break;
                    }

                }
            } */
           
            vectMEta
         
    }
    pub fn obtener_pagina_v4_invert(&self, from_index: u64, limit: u64) -> Vec<Meta>  {
      
        let mut vectMEta = vec![];
        assert_ne!(limit, 0, "Cannot provide limit of 0.");  //0 //30 -> 30:45
        //let mut counter: usize = from_index;                //46 //30 -> 30:89                      
        log!("  from  -> : {},to -> {}",&from_index,&limit);
        let mut _tokfound =0;  //0
        let totoal =  self.get_on_total_toks()-1;
                log!(" {}" ,&totoal);
            for x in from_index..totoal { 
                
                if  x>= totoal.clone() { //0 ->150   //29-> 150
                    break;
                }
                    let mut token =self.get_token(x.to_string().clone());
               
                 
                    if token.on_sale{
                        
                    vectMEta.push(token  );
                    _tokfound+=1;
                    
                     }
                if _tokfound== limit {  //1 <30  //30-30
                    break;
                }
                
                          
            }
            /* if vectMEta.len()< 12 && limit < totoal {
                let  newfrom =limit; //12
                let  newlimit = limit+30; //24
               
                for y in newfrom..newlimit { 
                 let mut token =self.get_token(y.to_string().clone());
                 log!("  2da vuetla token -> : {:?}",&y );
                     if token.on_sale{
                         
                        vectMEta.push(token  );
                        _tokfound+=1;
                        
                    }
                     if y== newlimit {
                        break;
                    }

                }
            } */
           
            vectMEta
         
    }

    pub fn tokens_of(&self, account_id: ValidAccountId,from_index: U128,limit: u64,) -> Vec<Token> {
        return self
            .tokens
            .nft_tokens_for_owner(account_id, Some(from_index), Some(limit));
    }
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        Self {
           
            tokens:old_state.tokens,
            metadata: old_state.metadata,
            n_total_tokens:old_state.n_total_tokens,
            n_token_on_sale: old_state.n_token_on_sale,
            n_token_on_auction:old_state.n_token_on_auction,
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


