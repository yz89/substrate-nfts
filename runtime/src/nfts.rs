use crate::linked_item::{LinkedItem, LinkedList};
use parity_codec::{Decode, Encode};
use rstd::result;
use runtime_io::blake2_256;
use runtime_primitives::traits::{As, Bounded, Member, One, SimpleArithmetic};
use support::{
    decl_module, decl_storage, dispatch::Result, ensure, Parameter, StorageMap, StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type NFTIndex: Parameter + Member + Default + SimpleArithmetic + Bounded + Copy;
}

type NFTokenId = [u8; 32];

#[derive(Encode, Decode, Clone)]
pub struct NFToken {
    pub token_id: NFTokenId,
    pub lifetime: u64,
}

type NFTLinkedItem<T> = LinkedItem<<T as Trait>::NFTIndex>;
type OwnedNFTsList<T> =
    LinkedList<OwnedNFTs<T>, <T as system::Trait>::AccountId, <T as Trait>::NFTIndex>;

decl_storage! {
    trait Store for Module<T: Trait> as NFTs {
        /// Stores all the NFTs, key is the NFToken Id
        pub IdToNFT get(id_to_nft): map NFTokenId => Option<NFToken>;
        /// Stores all the NFTs, key is the NFToken index
        pub IndexToNFT get(index_to_nft): map T::NFTIndex => Option<NFToken>;
        /// Stores the total number of NFTs. i.e. the next NFToken index
        pub NFTsCount get(nfts_count): T::NFTIndex;
        /// Get NFToken ownership. Stored in a linked map.
        pub OwnedNFTs get(owned_nfts): map (T::AccountId, Option<T::NFTIndex>) =>
        Option<NFTLinkedItem<T>>;
        /// Stores the NFT ownership
        pub IndexToOwner get(index_to_owner): map T::NFTIndex => Option<T::AccountId>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Handler called by the system on block finalization
        fn on_finalize(n: T::BlockNumber) {
            // burn expired tokens
            Self::burn_expired_tokens(As::as_(n));
        }

        /// Mint a new NFToken(for param test)
        pub fn mint_param_test(origin, token_id: NFTokenId, lifetime: u64) {
            let sender = ensure_signed(origin)?;
            Self::mint(&sender, token_id, lifetime)?;
        }

        /// Mint a new NFToken(for random test)
        pub fn mint_random_test(origin) {
            let sender = ensure_signed(origin)?;
            let nft_index = Self::next_nft_index()?;

            // Generate a random 128 bit value
            let token_id = Self::random_value(&sender);

            // Create and store nft
            let nft = NFToken{
                token_id,
                lifetime: As::as_(<system::Module<T>>::block_number()) + 10,
            };
            Self::insert_nft(&sender, nft_index, nft);
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn mint(
        sender: &T::AccountId,
        token_id: NFTokenId,
        lifetime: u64,
    ) -> result::Result<T::NFTIndex, &'static str> {
        let nft_index = Self::next_nft_index()?;
        let nft = NFToken { token_id, lifetime };
        Self::insert_nft(&sender, nft_index, nft);
        Ok(nft_index)
    }

    pub fn transfer(
        sender: &T::AccountId,
        recipient: &T::AccountId,
        nft_index: T::NFTIndex,
    ) -> Result {
        // Check if the nft exsit
        let transfer_nft = Self::index_to_nft(nft_index);
        ensure!(transfer_nft.is_some(), "Invalid transfer nft");

        // Check if the sender own this nft
        ensure!(
            <OwnedNFTs<T>>::exists(&(sender.clone(), Some(nft_index))),
            "Only owner can transfer nft"
        );

        <OwnedNFTsList<T>>::remove(&sender, nft_index);
        <OwnedNFTsList<T>>::append(&recipient, nft_index);
        <IndexToOwner<T>>::insert(nft_index, recipient);

        Ok(())
    }

    pub fn get_nft(nft_index: T::NFTIndex) -> Option<NFToken> {
        Self::index_to_nft(nft_index)
    }

    pub fn is_owned(owner: &T::AccountId, nft_index: T::NFTIndex) -> bool {
        <OwnedNFTs<T>>::exists(&(owner.clone(), Some(nft_index)))
    }

    pub fn owner_of(nft_index: T::NFTIndex) -> Option<T::AccountId> {
        Self::index_to_owner(nft_index)
    }

    fn random_value(sender: &T::AccountId) -> NFTokenId {
        let payload = (
            <system::Module<T>>::random_seed(),
            sender.clone(),
            <system::Module<T>>::extrinsic_index(),
            <system::Module<T>>::block_number(),
        );
        payload.using_encoded(blake2_256)
    }

    fn next_nft_index() -> result::Result<T::NFTIndex, &'static str> {
        let nft_index = Self::nfts_count();
        if nft_index == T::NFTIndex::max_value() {
            return Err("NFTs count overflow");
        }
        Ok(nft_index)
    }

    fn insert_owned_nft(owner: &T::AccountId, nft_index: T::NFTIndex) {
        <OwnedNFTsList<T>>::append(owner, nft_index);
        <IndexToOwner<T>>::insert(nft_index, owner);
    }

    fn insert_nft(owner: &T::AccountId, nft_index: T::NFTIndex, nft: NFToken) {
        // Create and store NFT
        <IdToNFT<T>>::insert(nft.token_id, nft.clone());
        <IndexToNFT<T>>::insert(nft_index, nft);
        <NFTsCount<T>>::put(nft_index + One::one());

        Self::insert_owned_nft(owner, nft_index);
    }

    fn burn_token(nft_index: T::NFTIndex, nft: NFToken) {
        <IdToNFT<T>>::remove(nft.token_id);
        <IndexToNFT<T>>::remove(nft_index);
        let count = Self::nfts_count();
        <NFTsCount<T>>::put(count - One::one());

        let owner = <IndexToOwner<T>>::get(nft_index);
        if owner.is_some() {
            <OwnedNFTsList<T>>::remove(&owner.unwrap(), nft_index);
        }
        <IndexToOwner<T>>::remove(nft_index);
    }

    fn burn_expired_tokens(num: u64) {
        runtime_io::print("Burning the token at height: ");
        runtime_io::print(num);
        let count = Self::nfts_count();
        for i in 0..As::as_(count) {
            let nft_index = T::NFTIndex::sa(i);
            let nft = Self::index_to_nft(nft_index);
            if nft.is_some() {
                let nft = nft.unwrap();
                if nft.lifetime < num {
                    Self::burn_token(nft_index, nft);
                }
            }
        }
    }
}
