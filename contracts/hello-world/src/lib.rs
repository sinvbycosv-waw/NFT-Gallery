/*#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;*/
#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// NFT structure to store NFT details
#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub token_id: u64,
    pub name: String,
    pub creator: Address,
    pub metadata_uri: String,
    pub category: String,
    pub price: u64,
    pub is_featured: bool,
    pub created_at: u64,
}

// Gallery statistics
#[contracttype]
#[derive(Clone)]
pub struct GalleryStats {
    pub total_nfts: u64,
    pub featured_nfts: u64,
    pub total_creators: u64,
}

// Enum for mapping NFT token_id to NFT data
#[contracttype]
pub enum NFTBook {
    NFT(u64)
}

// Symbol for storing gallery statistics
const GALLERY_STATS: Symbol = symbol_short!("G_STATS");

// Symbol for NFT counter
const NFT_COUNT: Symbol = symbol_short!("NFT_CNT");

#[contract]
pub struct NFTGalleryContract;

#[contractimpl]
impl NFTGalleryContract {
    
    // Function to add a new NFT to the gallery
    pub fn add_nft(
        env: Env, 
        name: String, 
        creator: Address, 
        metadata_uri: String, 
        category: String, 
        price: u64
    ) -> u64 {
        // Get current NFT count and increment
        let mut nft_count: u64 = env.storage().instance().get(&NFT_COUNT).unwrap_or(0);
        nft_count += 1;
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create new NFT instance
        let new_nft = NFT {
            token_id: nft_count,
            name: name.clone(),
            creator: creator.clone(),
            metadata_uri,
            category,
            price,
            is_featured: false,
            created_at: timestamp,
        };
        
        // Update gallery statistics
        let mut stats = Self::get_gallery_stats(env.clone());
        stats.total_nfts += 1;
        
        // Store NFT data
        env.storage().instance().set(&NFTBook::NFT(nft_count), &new_nft);
        env.storage().instance().set(&NFT_COUNT, &nft_count);
        env.storage().instance().set(&GALLERY_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "NFT Added with Token ID: {}", nft_count);
        nft_count
    }
    
    // Function to feature/unfeature an NFT (admin function)
    pub fn toggle_featured(env: Env, token_id: u64) {
        let mut nft = Self::get_nft_by_id(env.clone(), token_id);
        
        // Check if NFT exists
        if nft.token_id == 0 {
            log!(&env, "NFT not found!");
            panic!("NFT not found!");
        }
        
        // Toggle featured status
        let mut stats = Self::get_gallery_stats(env.clone());
        
        if nft.is_featured {
            nft.is_featured = false;
            stats.featured_nfts -= 1;
            log!(&env, "NFT {} removed from featured", token_id);
        } else {
            nft.is_featured = true;
            stats.featured_nfts += 1;
            log!(&env, "NFT {} added to featured", token_id);
        }
        
        // Update storage
        env.storage().instance().set(&NFTBook::NFT(token_id), &nft);
        env.storage().instance().set(&GALLERY_STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);
    }
    
    // Function to get NFT details by token ID
    pub fn get_nft_by_id(env: Env, token_id: u64) -> NFT {
        let key = NFTBook::NFT(token_id);
        
        env.storage().instance().get(&key).unwrap_or(NFT {
            token_id: 0,
            name: String::from_str(&env, "Not_Found"),
            creator: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            metadata_uri: String::from_str(&env, "Not_Found"),
            category: String::from_str(&env, "Not_Found"),
            price: 0,
            is_featured: false,
            created_at: 0,
        })
    }
    
    // Function to get gallery statistics
    pub fn get_gallery_stats(env: Env) -> GalleryStats {
        env.storage().instance().get(&GALLERY_STATS).unwrap_or(GalleryStats {
            total_nfts: 0,
            featured_nfts: 0,
            total_creators: 0,
        })
    }
}
