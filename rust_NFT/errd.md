near call dev-1636747327239-18935385243808 new_default_meta '{""owner_id"": ""dokxo.testnet""}' --accountId dev-1636747327239-18935385243808

near call dev-1636747327239-18935385243808 minar '{ "token_owner_id":"dokxo.testnet","token_metadata":"{"title":"nombre del token","description":"descripción","media":"imagenim","media_hash":"imageni"}"}' --accountId dev-1636747327239-18935385243808 --amount 0.1 

near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata":'{"title":"nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash":"imagenimagenimagenimagenimagenim"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": "dokxo.testnet", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}}'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": ""dokxo.testnet"", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}'" }'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar ‘{token_owner_id: “dokxo.testnet", “token_metadata”: { “title”: “nombre del token”, “description”: “descripción”, “media”: “imagenimagenimagenimagenimagenim”,“extra”:“{‘“hp”:“20",“attack”:“10",“defense”:“15",“speed”:“13"}‘“}}’ --accountId dev-1636747327239-18935385243808 --deposit 0.1


EL CHIDO CON EL QUE SÍ FUNCIONA LA MINACIÓN 

near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata":{"title":"nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash":"imagenimagenimagenimagenimagenim"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near deploy --wasmFile target/wasm32-unknown-unknown/release/nft_marketplace.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1636751893359-19496702378959


near call dev-1636751893359-19496702378959 minar '{"token_owner_id": "dev-1636751893359-19496702378959", "token_metadata": { "title": "Tenochtitlan", "description": "This is Tenochtitlan", "media": "imagenimagenimagenimagenimagenim","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'dev-1636751893359-19496702378959'","'price'":"'10'"}'"}}' --accountId dev-1636751893359-19496702378959 --amount 0.1

near call dev-1636751893359-19496702378959 get_token '{"token_id": "1"}' --accountId dev-1636751893359-19496702378959