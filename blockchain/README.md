`💻 Tecnologías utilizadas`
==========

para el desarrollo del smart contract se utilizo el siguiente stack de tecnologías:
1. truffle framework 
2. node.js versión 12 
3. Solidity
4. MetaMask (extensión del navegador y app móvil) 
5. red de [aurora]

`👨🏻‍💻 instalación local` 
=================

1. para instalar el proyecto de forma local primero tenemos que asegurarnos de tener node js instalado en nuestro equipo, para eso es recomendado usar la herramienta de [nvm] (node version manager) e instalamos la versión 12

```bash
nvm install 12.22.1
```
2. instalamos de forma global el framework de truffle
```bash
npm install -g truffle
```
3. instalamos las dependencias del archivo `package.json` dentro del directorio `/blockchain`
```bash
npm install 
```

`👩🏻‍💻 despliegue del smart contract en la red de` [aurora]
================

una vez instalado de forma local el proyecto es necesario desplegar el contrato en la red de [aurora], para esto primero tendremos que configurar el proyecto para usar nuestra address de la wallet de metamask asi como el seed phrase para esto configuraremos dos archivos:

1. configuramos el `truffle-config.js` dentro del directorio `/blockchain` y cambiamos el address por el propio 

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/cambiar_address.png?raw=true)

2. configuramos el archivo `.secret` dentro del directorio `/blockchain` y escribimos nuestra seed phrase (frase de 12 palabras) como texto plano 

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/seed_phrase.png?raw=true)

con todo configurado pasamos a ejecutar el siguiente `script` estando dentro del directorio `/blockchain` para desplegar el contrato
```bash
npm run deploy:aurora
```
este comando desplegará el smart contract en la red de [aurora] para que podamos interactuar posteriormente con él.

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/despliegue.png?raw=true)

`interactuar con el smart contract`
================

para interactuar con el smart contract desplegado es necesario hacer una instancia del contrato desde la consola de truffle para eso usaremos los siguientes comandos:

```bash
truffle console --network aurora
```

```bash
const cvt = await marketplace.deployed()
```

![picture](https://github.com/cristian-cloudmex/NFT-culturas-latinas/blob/master/assets/truffle.png?raw=true)

`📃 Métodos del smart contract`
===============
para ejecutar estos metodos primero debemos tener una `instancia del contrato corriendo en la red de aurora` 

Método para minar un nuevo token nft 
------------------

```bash
await cvt.minar("address", "data", precio)
```

Método para transferir un token nft 
-----------------
```bash
await cvt.transferirnft("address", tokenid)
```

Método para listar los token nft a la venta
-----------------
```bash
await cvt.obtenernfts()
```

Método para comprar un token nft
----------------
```bash
await cvt.comprarnft(tokenid)
```

Método para listar mis tokens nft adquiridos
---------------
```bash
await cvt.tokensof("address")
```

Método para quitar de la venta uno de mis token nft 
---------------
```bash
await cvt.quitardelmarketplace(tokenid)
```

Método para revender un token nft
---------------
```bash
await cvt.revender(tokenid, precio)
```

`arbol de archivos`
================

```bash
├── readme.md                                   #este archivo
├── contracts                                   #directorio con los smart contract en solidity
│   ├── marketplace.sol
│   └── migrations.sol
├── migrations                                  #migraciones del contrato
│   ├── 1_initial_migration.js
│   └── 2_market_place.js
├── package-lock.json
├── package.json                                #archivo de dependencias del proyecto
├── test                                        #directorio de los test del smart contract
│   └── market_place.js
├── truffle-config.js                           # archivo de configuración de truffle
├── utils
│   └── exception.js
└── yarn.lock
```

`🎥 tutoriales en video`
==============

En el canal en YouTube de [NEAR Hispano] se encuentra una lista de reproducción con el avance diario del proyecto a manera de documentación el cual se encuentra a continuación:

https://www.youtube.com/watch?v=9J2xkT_tFHk&list=PLixWO0N_iFTMGU3M5KHpuMqhpdMKzw88f

Ademas contamos con una lista de reproducción en donde se brinda un demo de las principales funcionalidades del proyecto, misma que se puede encontrar aqui:

https://www.youtube.com/watch?v=9PfxYtO0HK4&list=PLixWO0N_iFTOoCXL_rcyDowvxaO8BKAUD


[nvm]:  https://github.com/nvm-sh/nvm
[aurora]: https://aurora.dev/