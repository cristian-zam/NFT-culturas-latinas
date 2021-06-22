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
}
