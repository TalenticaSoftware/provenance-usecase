#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use sp_std::{prelude::*, vec::Vec};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, ensure};
use frame_system::ensure_signed;
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum MemberType {
	Manufacturer,
	Carrier,
	Retailer,
	Customer,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum BottleStatus {
	Manufactured,
	ShipmentRegistered,
	ShipmentInTransit,
	ShipmentDelivered,
	SoldToCustomer,
}

impl Default for BottleStatus {
	fn default() -> Self {
		BottleStatus::Manufactured
	}
}

pub const BOTTLE_ID_MAX_LENGTH: usize = 36;
pub type BottleId = Vec<u8>;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Bottle<AccountId, Moment> {
	id: BottleId,
	owner: AccountId,
	status: BottleStatus,
	registered: Moment,
}

impl<AccountId, Moment> Bottle<AccountId, Moment> {
	pub fn change_owner(mut self, new_owner: AccountId) -> Self {
		self.owner = new_owner;
		self
	}
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + timestamp::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as RegistrarModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		pub Members get(fn members): map hasher(identity) MemberType => Vec<T::AccountId>;
		
		pub Bottles get(fn bottle_by_id): map hasher(blake2_128_concat) BottleId => Option<Bottle<T::AccountId, T::Moment>>;
		pub BottlesOfManufacturer get(fn bottles_of_manufacturer): map hasher(blake2_128_concat) T::AccountId => Vec<BottleId>;
		pub ManufacturerOf get(fn owner_of): map hasher(blake2_128_concat) BottleId => Option<T::AccountId>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Manufacturer has been added. [account]
		ManufacturerAdded(AccountId),
		/// Carrier has been added. [account]
		CarrierAdded(AccountId),
		/// Retailer has been added. [account]
		RetailerAdded(AccountId),
		///Customer has been added. [account]
		CustomerAdded(AccountId),
		///Bottle has been registered. [account, bottleid]
		BottleRegistered(AccountId, BottleId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Member Account is already registered.
		MemberAlreadyExist,
		/// Please provide bottle id.
		BottleIdMissing,
		/// Bottle already exists.
		BottleIdExists,
		/// Bottle id too long.
		BottleIdTooLong,
		/// Not a manufacturer.
		NotManufacturer,
		/// Not a carrier.
		NotCarrier,
		/// Not a retailer.
		NotRetailer,
		/// Not a customer.
		NotCustomer,
		/// Bottle does not exist.
		BottleNotExist,
		/// Not the bottle manufacturer
		NotBottleManufacturer,
		// Not the bottle owner
		NotBottleOwner,		
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn register_manufacturer(origin) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::add_member(MemberType::Manufacturer, &who)?;

			// Emit an event.
			Self::deposit_event(Event::<T>::ManufacturerAdded(who));
			// Return a successful dispatch::DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn register_carrier(origin) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::add_member(MemberType::Carrier, &who)?;

			// Emit an event.
			Self::deposit_event(Event::<T>::CarrierAdded(who));
			// Return a successful dispatch::DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn register_retailer(origin) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::add_member(MemberType::Retailer, &who)?;

			// Emit an event.
			Self::deposit_event(Event::<T>::RetailerAdded(who));
			// Return a successful dispatch::DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn register_customer(origin) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::add_member(MemberType::Customer, &who)?;

			// Emit an event.
			Self::deposit_event(Event::<T>::CustomerAdded(who));
			// Return a successful dispatch::DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn register_bottle(origin, id: BottleId) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::validate_manufacturer(&who)?;

			Self::validate_bottle_id(&id)?;

			Self::validate_new_bottle(&id)?;

			let new_bottle = Self::new_bottle()
				.identified_by(id.clone())
				.manufactured_by(who.clone())
				.registered_on(<timestamp::Module<T>>::now())
				.build();

			Bottles::<T>::insert(&id, new_bottle);
			BottlesOfManufacturer::<T>::append(&who, &id);
			ManufacturerOf::<T>::insert(&id, &who);

			Self::deposit_event(Event::<T>::BottleRegistered(who, id));

			Ok(())
		}
	}

}

impl<T: Trait> Module<T> {
	fn add_member(member_type: MemberType, account_id: &T::AccountId) -> dispatch::DispatchResult {
		let member_itr = Members::<T>::iter();

		for mem in member_itr {
			ensure!(!(mem.1).contains(account_id), Error::<T>::MemberAlreadyExist);
		}

		let mut members = <Module<T>>::members(&member_type);
		
		members.push(account_id.clone());
		Members::<T>::insert(member_type, members);

		Ok(())
	}

	pub fn validate_manufacturer(account_id: &T::AccountId) -> dispatch::DispatchResult {
		let all_manufacturers = <Module<T>>::members(MemberType::Manufacturer);

		ensure!(all_manufacturers.contains(account_id), Error::<T>::NotManufacturer);

		Ok(())
	}

	pub fn validate_carrier(account_id: &T::AccountId) -> dispatch::DispatchResult {
		let all_carriers = <Module<T>>::members(MemberType::Carrier);

		ensure!(all_carriers.contains(account_id), Error::<T>::NotCarrier);

		Ok(())
	}

	pub fn validate_retailer(account_id: &T::AccountId) -> dispatch::DispatchResult {
		let all_retailers = <Module<T>>::members(MemberType::Retailer);

		ensure!(all_retailers.contains(account_id), Error::<T>::NotRetailer);

		Ok(())
	}

	pub fn validate_customer(account_id: &T::AccountId) -> dispatch::DispatchResult {
		let all_customers = Module::<T>::members(MemberType::Customer);

		ensure!(all_customers.contains(account_id), Error::<T>::NotManufacturer);

		Ok(())
	}

	pub fn check_bottle_id_present(id: &[u8]) -> dispatch::DispatchResult {
		ensure!(
			<Bottles::<T>>::contains_key(id),
			Error::<T>::BottleNotExist
		);
		Ok(())
	}

	// pub fn check_bottle_manufacturer(id: &[u8], account: &T::AccountId) -> dispatch::DispatchResult {
	// 	ensure!(<ManufacturerOf::<T>>::get(id) == Some(account.clone()), Error::<T>::NotBottleManufacturer);

	// 	Ok(())
	// }

	pub fn validate_bottle_id(id: &[u8]) -> dispatch::DispatchResult {
		ensure!(!id.is_empty(), Error::<T>::BottleIdMissing);
		ensure!(id.len() <= BOTTLE_ID_MAX_LENGTH, Error::<T>::BottleIdTooLong);
		Ok(())
	}

	pub fn validate_new_bottle(id: &[u8]) -> dispatch::DispatchResult {
		// Bottle existence check
		ensure!(
			!<Bottles::<T>>::contains_key(id),
			Error::<T>::BottleIdExists
		);
		Ok(())
	}

	pub fn new_bottle() -> BottleBuilder<T::AccountId, T::Moment> {
        BottleBuilder::<T::AccountId, T::Moment>::default()
    }

	pub fn update_bottle_owner(bottle_id: &BottleId, new_owner: T::AccountId) -> dispatch::DispatchResult {

		let mut bottle: Bottle<T::AccountId, T::Moment> = match Bottles::<T>::get(bottle_id) {
			None => Err(Error::<T>::BottleNotExist),
			Some(bottle) => Ok(bottle),
		}?;

		bottle = bottle.change_owner(new_owner);

		Bottles::<T>::insert(bottle_id, bottle);

		Ok(())
	}

	pub fn check_bottle_owner(id: &[u8], owner: T::AccountId) -> dispatch::DispatchResult {
		ensure!(<ManufacturerOf::<T>>::get(id) == Some(owner), Error::<T>::NotBottleOwner);

		Ok(())
	}
}


#[derive(Default)]
pub struct BottleBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    id: BottleId,
    owner: AccountId,
	status: BottleStatus,
    registered: Moment,
}

impl<AccountId, Moment> BottleBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    pub fn identified_by(mut self, id: BottleId) -> Self {
        self.id = id;
        self
    }

    pub fn manufactured_by(mut self, manufacturer: AccountId) -> Self {
        self.owner = manufacturer;
        self
    }

    pub fn registered_on(mut self, registered: Moment) -> Self {
        self.registered = registered;
        self
    }

    pub fn build(self) -> Bottle<AccountId, Moment> {
        Bottle::<AccountId, Moment> {
            id: self.id,
            owner: self.owner,
            registered: self.registered,
			status: BottleStatus::Manufactured,
        }
    }
}