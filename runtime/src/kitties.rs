use crate::nfts;
use parity_codec::Encode;
use rstd::result;
use runtime_io::blake2_256;
use support::{decl_event, decl_module, decl_storage, ensure, traits::Currency, StorageMap};
use system::ensure_signed;

pub trait Trait: nfts::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub KittyPrices get(kitty_price): map T::NFTIndex => Option<BalanceOf<T>>;
    }
}

decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
		<T as nfts::Trait>::NFTIndex,
		Balance = BalanceOf<T>,
	{
		/// A kitty is created. (owner, kitty_index)
		Created(AccountId, NFTIndex),
		/// A kitty is transferred. (from, to, kitty_index)
		Transferred(AccountId, AccountId, NFTIndex),
		/// A kitty is available for sale. (owner, kitty_index, price)
		Ask(AccountId, NFTIndex, Option<Balance>),
		/// A kitty is sold. (from, to, kitty_index, price)
		Sold(AccountId, AccountId, NFTIndex, Balance),
	}
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        /// Create a new kitty
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;

            // Generate a random 128 bit value
            let dna = Self::random_value(&sender);

            // Create and store kitty
            let lifetime = 1000;
            let kitty_index = <nfts::Module<T>>::mint(&sender, dna, lifetime)?;

            Self::deposit_event(RawEvent::Created(sender, kitty_index));
        }

        /// Transfer kitty
        pub fn transfer(origin, recipient: T::AccountId, kitty_index: T::NFTIndex) {
            let sender = ensure_signed(origin)?;
            <nfts::Module<T>>::transfer(&sender, &recipient, kitty_index)?;

            Self::deposit_event(RawEvent::Transferred(sender, recipient, kitty_index));
        }

        /// Breed kitties
        pub fn breed(origin, kitty_index_1: T::NFTIndex, kitty_index_2: T::NFTIndex) {
            let sender = ensure_signed(origin)?;
            let new_kitty_id = Self::do_breed(&sender, kitty_index_1, kitty_index_2)?;

            Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
        }

        pub fn ask(origin, kitty_index: T::NFTIndex, price: Option<BalanceOf<T>>) {
            let sender = ensure_signed(origin)?;

            ensure!(<nfts::Module<T>>::is_owned(&sender, kitty_index), "Only owner can set price for kitty");

            if let Some(price) = price {
                <KittyPrices<T>>::insert(kitty_index, price);
            } else {
                <KittyPrices<T>>::remove(kitty_index);
            }

            Self::deposit_event(RawEvent::Ask(sender, kitty_index, price));
        }

        pub fn buy(origin, kitty_index: T::NFTIndex, price: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;

            let owner = <nfts::Module<T>>::owner_of(kitty_index);
            ensure!(owner.is_some(), "Kitty does not exist");
            let owner = owner.unwrap();

            let kitty_price = Self::kitty_price(kitty_index);
            ensure!(kitty_price.is_some(), "Kitty not for sale");
            let kitty_price = kitty_price.unwrap();

            ensure!(price >= kitty_price, "Price is too low");

            T::Currency::transfer(&sender, &owner, kitty_price)?;

            <KittyPrices<T>>::remove(kitty_index);

            <nfts::Module<T>>::transfer(&owner, &sender, kitty_index)?;

            Self::deposit_event(RawEvent::Sold(owner, sender, kitty_index, kitty_price));
        }
    }
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    ((selector & dna1) | (!selector & dna2))
}

impl<T: Trait> Module<T> {
    fn random_value(sender: &T::AccountId) -> [u8; 32] {
        let payload = (
            <system::Module<T>>::random_seed(),
            sender.clone(),
            <system::Module<T>>::extrinsic_index(),
            <system::Module<T>>::block_number(),
        );
        payload.using_encoded(blake2_256)
    }

    fn do_breed(
        sender: &T::AccountId,
        kitty_index_1: T::NFTIndex,
        kitty_index_2: T::NFTIndex,
    ) -> result::Result<T::NFTIndex, &'static str> {
        ensure!(kitty_index_1 != kitty_index_2, "Needs different parents");

        let kitty1 = <nfts::Module<T>>::get_nft(kitty_index_1);
        let kitty2 = <nfts::Module<T>>::get_nft(kitty_index_2);

        ensure!(kitty1.is_some(), "Invalid kitty_index_1");
        ensure!(kitty2.is_some(), "Invalid kitty_index_2");

        let kitty1_dna = kitty1.unwrap().token_id;
        let kitty2_dna = kitty2.unwrap().token_id;
        let selector = Self::random_value(&sender);

        let mut new_dna = [0u8; 32];
        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        let lifetime = 1000;
        let kitty_index = <nfts::Module<T>>::mint(&sender, new_dna, lifetime)?;

        Ok(kitty_index)
    }
}
