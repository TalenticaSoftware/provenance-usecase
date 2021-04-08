use frame_support::sp_std::prelude::*;
use crate::types::{Shipment, ShipmentStatus, ShipmentId};
use registrar::BottleId;


// Shipment Builder

#[derive(Default)]
pub struct ShipmentBuilder<AccountId, Moment> 
where 
    AccountId: Default,
    Moment: Default,
{
    pub id: ShipmentId,
    pub manufacturer: AccountId,
    pub carrier: AccountId,
    pub retailer: AccountId,
    pub bottles: Vec<BottleId>,
    pub registered: Moment,
}

impl<AccountId, Moment> ShipmentBuilder<AccountId, Moment> 
where 
    AccountId: Default,
    Moment: Default,
{
    pub fn identified_by(mut self, id: ShipmentId) -> Self {
        self.id = id;
        self
    }

    pub fn manufactured_by(mut self, account: AccountId) -> Self {
        self.manufacturer = account;
        self
    }

    pub fn carried_by(mut self, account: AccountId) -> Self {
        self.carrier = account;
        self
    }

    pub fn sent_to(mut self, account: AccountId) -> Self {
        self.retailer = account;
        self
    }

    pub fn with_bottles(mut self, bottles: Vec<BottleId>) -> Self {
        self.bottles = bottles;
        self
    }

    pub fn registered_at(mut self, registered_at: Moment) -> Self {
        self.registered = registered_at;
        self
    }

    pub fn build(self) -> Shipment<AccountId, Moment> {
        Shipment::<AccountId, Moment> {
            id: self.id,
            manufacturer: self.manufacturer,
            carrier: self.carrier,
            retailer: self.retailer,
            bottles: self.bottles,
            status: ShipmentStatus::Pending,
            registered: self.registered,
            delivered: None,
        }
    }

}
