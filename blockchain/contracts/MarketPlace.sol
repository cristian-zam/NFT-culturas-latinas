// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract MarketPlace is ERC721URIStorage {
    using Counters for Counters.Counter;
    //representa un entero que lleva la cuenta de los tokensNFT
    Counters.Counter private _tokenIds;

    //matenera el dueño del contrato o quien lo mino
    address public minero;

    constructor() ERC721("MarketPlace", "MPE") {
        minero = msg.sender;
    }

    /**
	@param receptor es a quien mandaremos el  token 
	@param datos es un json con todos los datos del token
	@return {uint256} equivale a el tokenid recien creado
	 */
    function minar(address receptor, string memory datos)
        public
        returns (uint256)
    {
        //aumentar en 1 el conteo de tokens
        _tokenIds.increment();
        //obtener el tokenid actual
        uint256 newItemId = _tokenIds.current();
        //minar el token
        _mint(receptor, newItemId);
        //asignarle los metadatos
        _setTokenURI(newItemId, datos);

        return newItemId;
    }

    /**
    * Función para transferir un token NFT del propietario a otro address
    @param to representa a quien vamos a transferir nuestro NFT
    @param tokenId este valor indica el id que vamos a transferir 
    @return {addresss}  esta función nos retorna el address del nuevo propietario del NFT
     */

    function transferArt(address to, uint256 tokenId) public returns (address) {
        require(_exists(tokenId), "El token que intentas transferir no existe");
        require(msg.sender == ownerOf(tokenId), "Este token no te pertenece");

        //transferir el token NFT si el token id existe y es nuestro
        safeTransferFrom(msg.sender, to, tokenId);
        //retornar el nuevo owner del token NFT
        return to;
    }

    /** 
    * Función para eliminar un token NFT en tu address
    @param tokenID representa el id del token NFT a eliminar
    @return {success} si el proceso fue llevado con exito la funcion retorna un true
     */
    function burnArt(uint256 tokenId) public returns (bool success) {
        require(_exists(tokenId), "El token que intentas quemar no existe");
        require(msg.sender == ownerOf(tokenId), "Este token no te pertenece");

        //Eliminamos el token NFT si el token id existe y es nuestro
        _burn(tokenId);

        return true;
    }
}
