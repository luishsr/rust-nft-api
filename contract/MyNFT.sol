// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract MyNFT is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    struct TokenDetails {
        uint256 tokenId;
        string tokenName;
        address tokenOwner;
        string tokenURI;
    }

    mapping(uint256 => TokenDetails) private _tokenDetails;
    mapping(address => uint256[]) private _ownedTokens;
    mapping(uint256 => uint256) private _ownedTokensIndex;  // Maps token ID to its index in the owner's token list

    constructor() ERC721("MyNFT", "MNFT") {}

    function mintNFT(address recipient, string memory tokenName, string memory tokenURI) public returns (uint256) {
        _tokenIds.increment();
        uint256 newItemId = _tokenIds.current();
        _mint(recipient, newItemId);
        _setTokenURI(newItemId, tokenURI);

        _tokenDetails[newItemId] = TokenDetails({
            tokenId: newItemId,
            tokenName: tokenName,
            tokenOwner: recipient,
            tokenURI: tokenURI
        });

        _addTokenToOwnerEnumeration(recipient, newItemId);

        return newItemId;
    }

    function _addTokenToOwnerEnumeration(address to, uint256 tokenId) private {
        _ownedTokens[to].push(tokenId);
        _ownedTokensIndex[tokenId] = _ownedTokens[to].length - 1;
    }

    function getAllTokensByOwner(address owner) public view returns (uint256[] memory) {
        if (owner == address(0)) {
            uint256 totalTokens = _tokenIds.current();
            uint256[] memory allTokenIds = new uint256[](totalTokens);
            for (uint256 i = 0; i < totalTokens; i++) {
                allTokenIds[i] = i + 1;  // Token IDs are 1-indexed because of the way they are minted
            }
            return allTokenIds;
        } else {
            return _ownedTokens[owner];
        }
    }


    function getTokenDetails(uint256 tokenId) public view returns (uint256, string memory, address, string memory) {
        require(_ownerOf(tokenId) != address(0), "ERC721: Query for nonexistent token");

        TokenDetails memory tokenDetail = _tokenDetails[tokenId];
        return (tokenDetail.tokenId, tokenDetail.tokenName, tokenDetail.tokenOwner, tokenDetail.tokenURI);
    }

}