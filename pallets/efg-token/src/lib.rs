#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    #[pallet::pallet]
    pub struct Pallet<T>(_);
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        // Event definition
        type RuntimeEvent: From<Event<Self>>
        + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // Module Id
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }
    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T> = StorageValue<_, u64, ValueQuery>;
    #[pallet::storage]
    #[pallet::getter(fn balance_of)]
    pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Tokens were minted.
        Minted(T::AccountId, u64),
        /// Tokens were transferred.
        Transferred(T::AccountId, T::AccountId, u64),
    }
    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient balance.
        InsufficientBalance,
        /// Amount overflow.
        AmountOverflow,
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn mint(origin: OriginFor<T>, amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let new_total_supply = TotalSupply::<T>::get()
                .checked_add(amount)
                .ok_or(Error::<T>::AmountOverflow)?;
            TotalSupply::<T>::put(new_total_supply);
            Balances::<T>::mutate(&sender, |balance| -> DispatchResult {
                *balance = balance.checked_add(amount).ok_or(Error::<T>::AmountOverflow)?;
                Ok(())
            })?;
            Self::deposit_event(Event::Minted(sender, amount));
            Ok(())
        }
        #[pallet::weight(10_000)]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Balances::<T>::try_mutate(&sender, |balance| -> DispatchResult {
                let new_balance = balance.checked_sub(amount).ok_or(Error::<T>::InsufficientBalance)?;
                *balance = new_balance;
                Ok(())
            })?;
            Balances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
            Self::deposit_event(Event::Transferred(sender, to, amount));
            Ok(())
        }
    }
}