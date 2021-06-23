// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract MarketPlace is ERC721Enumerable {
    using Counters for Counters.Counter;
    //representa un entero que lleva la cuenta de los tokensNFT
    Counters.Counter private _tokenIds;

    //matenera el dueño del contrato o quien lo mino
    address public minero;

    // es la representacion de un token
    struct tokenData {
        uint256 price;
        string data;
        bool onSale;
        uint256 tokenID;
    }

    //contiene todas las representacion de cada token
    mapping(uint256 => tokenData) public tokensData;

    constructor() ERC721("MarketPlace", "MPE") {
        minero = msg.sender;
    }

    /**
    mina nuevos tokens
	@param receptor {address} es a quien mandaremos el  token 
	@param datos    {string} es un json con todos los datos del token
	@param price    {uint256} the price in weis
    @return {uint256} equivale a el tokenid recien creado
	 */
    function minar(
        address receptor,
        string memory datos,
        uint256 price
    ) public returns (uint256) {
        //obtener el tokenid actual
        uint256 newItemId = _tokenIds.current();

        //aumentar en 1 el conteo de tokens
        _tokenIds.increment();

        //minar el token
        _mint(receptor, newItemId);

        //agregar los datos a nuestro mapping
        tokensData[newItemId].data = datos;
        tokensData[newItemId].price = price;
        tokensData[newItemId].onSale = true;
        tokensData[newItemId].tokenID = newItemId;

        return newItemId;
    }

    /**
    * Función para transferir un token NFT del propietario a otro address
    @param to {address} representa a quien vamos a transferir nuestro NFT
    @param tokenId {uint256} este valor indica el id que vamos a transferir 
    @return {addresss}  esta función nos retorna el address del nuevo propietario del NFT
     */

    function transferirNft(address to, uint256 tokenId)
        public
        returns (address)
    {
        require(_exists(tokenId), "El token que intentas transferir no existe");
        require(msg.sender == ownerOf(tokenId), "Este token no te pertenece");

        //transferir el token NFT si el token id existe y es nuestro
        safeTransferFrom(msg.sender, to, tokenId);
        //retornar el nuevo owner del token NFT
        return to;
    }

    /** 
    * Función para eliminar un token NFT en tu address
    @param tokenId {uint256} representa el id del token NFT a eliminar
    @return {success} si el proceso fue llevado con exito la funcion retorna un true
     */
    function quemarNft(uint256 tokenId) public returns (bool) {
        require(_exists(tokenId), "El token que intentas quemar no existe");
        require(msg.sender == ownerOf(tokenId), "Este token no te pertenece");

        //Eliminamos el token NFT si el token id existe y es nuestro
        _burn(tokenId);

        return true;
    }

    /**
     * pone a la venta un token y si le das el precio lo cambia
     *@param tokenid {uint256} representa el identificador del tokenid
     *@param price  {uint256} representa el precio en wei al que se vendera
     *
     */
    function revender(uint256 tokenid, uint256 price) public {
        require(
            ownerOf(tokenid) == msg.sender,
            "solo el dueno del token puede venderlo"
        );
        tokensData[tokenid].onSale = true;

        if (!(price == 0)) {
            tokensData[tokenid].price = price;
        }
    }

    /**
     *cambia el estado del token para que este no pueda ser visualizado o vendido
     *@param tokenid {uint256} tokenid representa el token al cual afectar
     */
    function quitarDelMarketPlace(uint256 tokenid) public {
        //este require solo permite que el owner del tokenid pueda usar la funcion
        require(
            ownerOf(tokenid) == msg.sender,
            "solo el dueno del token puede venderlo"
        );
        // impide que accedamos a una posicion inexistente
        require(
            tokenid < _tokenIds.current(),
            "ese token todavia no ha sido creado"
        );
        tokensData[tokenid].onSale = false;
    }

    /**
     *regresa todos los nft disponibles
     *@return tokenData[] contiene todos los tokens disponibles
     */
    function obtenerNfts() public view returns (tokenData[] memory) {
        //es el numero de tokens
        uint256 nTokens = _tokenIds.current();
        //es un arreglo temporal con todos los nfts
        tokenData[] memory onSaleTokensData = new tokenData[](nTokens);
        //sacamos los nft del mapping y los almacenamos en el arreglo
        for (uint256 index = 0; index < nTokens; index++) {
            onSaleTokensData[index] = tokensData[index];
        }

        return onSaleTokensData;
    }

    /**
     *con esta funcion cualquiera puede comprar el nft que desea
     *@param tokenId {uint256} tokenid representa el tokenid a comprar
     */
    function comprarNft(uint256 tokenId) public payable {
        //este require se asegura que se mando el suficiente ether para comprar el token
        require(msg.value == tokensData[tokenId].price, "fondos insuficientes");
        //se impide la venta si no esta en venta el token
        require(
            tokensData[tokenId].onSale,
            "no se encuentra a la venta ese token"
        );
        //es el dueño del token
        address payable tokenOwner = payable(ownerOf(tokenId));

        //mandamos el token al comprador
        _transfer(tokenOwner, msg.sender, tokenId);

        //le transferimos el dinero al  vendedor
        tokenOwner.transfer(msg.value);

        //poner el token en pausa
        tokensData[tokenId].onSale = false;
    }

    /**
     * @param owner {address}  representa la dirección al cual consultar los tokens
     * @return tokenData[] arreglo con todos los datos de cada token
     */
    function tokensOf(address owner) public view returns (tokenData[] memory) {
        //numero de tokens
        uint256 nTokens = balanceOf(owner);
        if (nTokens == 0) {
            return (new tokenData[](nTokens));
        }
        //arreglo con los tokens
        tokenData[] memory userTokens = new tokenData[](nTokens);

        for (uint256 i = 0; i < nTokens; i++) {
            //obtener los datos de cada uno de los tokens
            userTokens[i] = tokensData[tokenOfOwnerByIndex(owner, i)];
        }
        return (userTokens);
    }
}
