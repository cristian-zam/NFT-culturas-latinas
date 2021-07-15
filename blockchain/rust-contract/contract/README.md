# Rust smart contract 

# Comandos

## Hacer login con el NEAR CLI
`near login`

## Almacenar el id de near como variable 
`ID=<tu near id>`

## Construir el smart contract para WebAssambly 
`cargo build --target wasm32-unknown-unknown`

## Desplegar contrato en testnet
`near dev-deploy --wasmFile target/wasm32-unknown-unknown/debug/greeter.wasm `

## Minar un token 
`near call <dirección del contrato> minar '{"token_id": "0", "token_owner_id": "'$ID'", "token_metadata": { "title": "Primer token", "description": "tToken 1"}}' --accountId $ID`

## Obtener el numero de tokens en el contrato
`near view <direccion del contrato> numero_de_nfts`

## Buscar un token mediante su token id
`near view <direccion del contrato> buscar_nft '{"token_id": "token_id"}'`

## Obtener los tokens NFT mediante un rango (token id de inicio y token id del final)
near view <dirección del contrato> nfts_por_propietario '{"account_id": "<NEAR ID del propietario de los tokens>", "from_index": "<id token inicio>", "limit": <id token limite>}'

## Obtener todos los tokens NFT del contrato
`near call <dirección del contrato> tokens_nfts '{"from_index": "0", "limit": 20}' --accountId $ID`


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
