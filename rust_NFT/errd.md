near call dev-1636747327239-18935385243808 new_default_meta '{""owner_id"": ""dokxo.testnet""}' --accountId dev-1636747327239-18935385243808

near call dev-1636747327239-18935385243808 minar '{
"token_owner_id":"dokxo.testnet",
"token_metadata":"{"title":"nombre del token","description":"descripción","media":"imagenim","media_hash":"imageni"}" 
}' --accountId dev-1636747327239-18935385243808 --amount 0.1 

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": "dokxo.testnet", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}}'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar "'{"token_owner_id": ""dokxo.testnet"", "token_metadata": "'{ "title": "nombre del token","description": "descripción","media": "imagenimagenimagenimagenimagenim","media_hash": "imagenimagenimagenimagenimagenim"}'" }'" --accountId dev-1636747327239-18935385243808 --amount 0.1

near call dev-1636747327239-18935385243808 minar ‘{token_owner_id: “dokxo.testnet", “token_metadata”: { “title”: “nombre del token”, “description”: “descripción”, “media”: “imagenimagenimagenimagenimagenim”,“extra”:“{‘“hp”:“20",“attack”:“10",“defense”:“15",“speed”:“13"}‘“}}’ --accountId dev-1636747327239-18935385243808 --deposit 0.1