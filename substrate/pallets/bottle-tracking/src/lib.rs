#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure,
	dispatch, traits::Get, sp_std::prelude::*, sp_std::vec::Vec};
use frame_system::ensure_signed;
use registrar::{self as registrar, BottleId};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;
use crate::types::*;

mod builders;
use crate::builders::*;

pub const SHIPMENT_ID_MAX_LENGTH: usize = 36;
pub const SHIPMENT_MAX_BOTTLES: usize = 5;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + timestamp::Trait + registrar::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as BottleTracking {
		pub Shipments: map hasher(blake2_128_concat) ShipmentId => Option<Shipment<T::AccountId, T::Moment>>;
		pub ShipmentsOfManufacturer: map hasher(blake2_128_concat) T::AccountId => Vec<ShipmentId>;
		pub ShipmentsOfCarrier: map hasher(blake2_128_concat) T::AccountId => Vec<ShipmentId>;
		pub ShipmentsOfRetailer: map hasher(blake2_128_concat) T::AccountId => Vec<ShipmentId>;
		pub BottleOfShipment: map hasher(blake2_128_concat) BottleId => Option<ShipmentId>;

		pub BottlesOfCustomer: map hasher(blake2_128_concat) T::AccountId => Vec<BottleId>;
		pub BottleSoldTo: map hasher(blake2_128_concat) BottleId => Option<T::AccountId>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Shipment registered. [shipment_id, manufacturer]
		ShipmentRegistered(ShipmentId, AccountId),
		/// Shipment status updated. [shipment_id, carrier, status]
		ShipmentStatusUpdated(ShipmentId, AccountId, ShipmentStatus),
		/// Bottles sold to customer. [customer]
		BottlesSoldToCustomer(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ShipmentIdExists,
		ShipmentIdMissing,
		ShipmentIdTooLong,
		ShipmentHasNoBottles,
		ShipmentHasTooManyBottles,
		BottleAlreadyShipped,
		ShipmentDoesNotExist,
		ShipmentHasBeenDelivered,
		ShipmentInTransit,
		NotShipmentCarrier,
		BottleNotShipped,
		ShipmentPending,
		NotBottleOwner,
		BottleAlreadySold,
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

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn register_shipment(
			origin,
			id: ShipmentId,
			carrier: T::AccountId,
    	 	retailer: T::AccountId,
			bottles: Vec<BottleId>,
		) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let manufacturer = ensure_signed(origin)?;

			Self::validate_shipment_id(&id)?;

			Self::validate_new_shipment(&id)?;

			registrar::Module::<T>::validate_manufacturer(&manufacturer)?;

			registrar::Module::<T>::validate_carrier(&carrier)?;

			registrar::Module::<T>::validate_retailer(&retailer)?;

			Self::validate_shipment_bottles(&bottles, &manufacturer)?;

			let shipment = Self::new_shipment()
				.identified_by(id.clone())
				.manufactured_by(manufacturer.clone())
				.carried_by(carrier.clone())
				.sent_to(retailer.clone())
				.with_bottles(bottles)
				.registered_at(<timestamp::Module<T>>::now())
				.build();

			for bottle in &shipment.bottles {
				BottleOfShipment::insert(&bottle, &id);
			}

			Shipments::<T>::insert(&id, shipment);
			ShipmentsOfManufacturer::<T>::append(&manufacturer, &id);
			ShipmentsOfCarrier::<T>::append(&carrier, &id);
			ShipmentsOfRetailer::<T>::append(&retailer, &id);

			// Emit an event.
			Self::deposit_event(RawEvent::ShipmentRegistered(id, manufacturer));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn track_shipment(
			origin,
			id: ShipmentId,
			operation: ShipmentOperation,	
		) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			registrar::Module::<T>::validate_carrier(&who)?;

			Self::validate_shipment_id(&id)?;

			let mut shipment = match Shipments::<T>::get(&id) {
				None => Err(Error::<T>::ShipmentDoesNotExist),
				Some(sp) => match sp.status {
					ShipmentStatus::Delivered => Err(Error::<T>::ShipmentHasBeenDelivered),
					ShipmentStatus::InTransit if operation == ShipmentOperation::Pickup => 
						Err(Error::<T>::ShipmentInTransit),
					_ => Ok(sp),
				}
			}?;

			ensure!(shipment.carrier == who, Error::<T>::NotShipmentCarrier);

			shipment = match operation {
				ShipmentOperation::Pickup => shipment.pickup(),
				ShipmentOperation::Deliver => shipment.delivered(<timestamp::Module<T>>::now()),
				_ => shipment,
			};

			if operation != ShipmentOperation::Scan {
				let status = shipment.status.clone();
				Shipments::<T>::insert(&id, shipment);
				Self::deposit_event(RawEvent::ShipmentStatusUpdated(id, who, status));
			}

			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn sell_to_customer(
			origin,
			customer: T::AccountId,
			bottles: Vec<BottleId>,
		) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			registrar::Module::<T>::validate_retailer(&who)?;

			registrar::Module::<T>::validate_customer(&customer)?;

			for bottle in &bottles {
				Self::is_bottle_sold(&bottle)?;
				Self::validate_bottle_owner(bottle, &who)?;
			}

			for bottle in &bottles {
				BottlesOfCustomer::<T>::append(&customer, &bottle);
				BottleSoldTo::<T>::insert(&bottle, &customer);
			}

			Self::deposit_event(RawEvent::BottlesSoldToCustomer(customer));

			Ok(())
		}

	}
}

impl<T: Trait> Module<T> {
	pub fn new_shipment() -> ShipmentBuilder<T::AccountId, T::Moment> {
		ShipmentBuilder::<T::AccountId, T::Moment>::default()
	}

	pub fn validate_new_shipment(id: &ShipmentId) -> dispatch::DispatchResult {
		ensure!(
			!Shipments::<T>::contains_key(id.clone()), 
			Error::<T>::ShipmentIdExists
		);
		Ok(())
	}

	pub fn validate_shipment_id(id: &[u8]) -> dispatch::DispatchResult {
		ensure!(!id.is_empty(), Error::<T>::ShipmentIdMissing);
		ensure!(id.len() <= SHIPMENT_ID_MAX_LENGTH, Error::<T>::ShipmentIdTooLong);
		Ok(())
	}

	pub fn validate_shipment_bottles(bottles: &[BottleId], manufacturer: &T::AccountId) -> dispatch::DispatchResult {

		ensure!(
			bottles.len() > 0,
			Error::<T>::ShipmentHasNoBottles,
		);

		ensure!(
            bottles.len() <= SHIPMENT_MAX_BOTTLES,
            Error::<T>::ShipmentHasTooManyBottles,
        );

		for bottle in bottles {
			registrar::Module::<T>::check_bottle_id_present(&bottle)?;
			registrar::Module::<T>::check_bottle_manufacturer(&bottle, manufacturer)?;
			ensure!(
				!BottleOfShipment::contains_key(bottle.clone()), 
				Error::<T>::BottleAlreadyShipped
			);
		}

        Ok(())
    }

	pub fn is_bottle_sold(bottle_id: &BottleId) -> dispatch::DispatchResult{
		match BottleSoldTo::<T>::get(bottle_id) {
			None => Ok(()),
			_ => Err(Error::<T>::BottleAlreadySold)?,
		}
	}


	pub fn validate_bottle_owner(bottle_id: &BottleId, account: &T::AccountId) -> dispatch::DispatchResult {
		
		registrar::Module::<T>::check_bottle_id_present(bottle_id)?;
		
		let shipment_id: ShipmentId = match BottleOfShipment::get(bottle_id) {
			None => Err(Error::<T>::BottleNotShipped),
			Some(sp) => Ok(sp),
		}?;

		match Shipments::<T>::get(&shipment_id) {
			None => Err(Error::<T>::ShipmentDoesNotExist)?,
			Some(sp) => match sp.status {
				ShipmentStatus::Pending => Err(Error::<T>::ShipmentPending)?,
				ShipmentStatus::InTransit => Err(Error::<T>::ShipmentInTransit)?,
				ShipmentStatus::Delivered if sp.retailer == *account => Ok(()),
				_ => Err(Error::<T>::NotBottleOwner)?,
			}
		}
	}
}
