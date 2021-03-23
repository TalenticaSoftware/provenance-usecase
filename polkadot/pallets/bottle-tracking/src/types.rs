use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*, sp_std::vec::Vec};
use codec::{Encode, Decode};
use registrar::{BottleId, BottleStatus};

pub type ShipmentId = Vec<u8>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum ShipmentStatus {
    Pending,
    InTransit,
    Delivered,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum ShipmentOperation {
    Pickup,
    Scan,
    Deliver,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Shipment<AccountId, Moment> {
    pub id: ShipmentId,
    pub manufacturer: AccountId,
    pub carrier: AccountId,
    pub retailer: AccountId,
    pub bottles: Vec<BottleId>,
    pub status: ShipmentStatus,
    pub registered: Moment,
    pub delivered: Option<Moment>,
}

impl<AccountId, Moment> Shipment<AccountId, Moment> {
    pub fn pickup(mut self) -> Shipment<AccountId, Moment> {
        self.status = ShipmentStatus::InTransit;
        self
    }

    pub fn delivered(mut self, when: Moment) -> Shipment<AccountId, Moment> {
        self.status = ShipmentStatus::Delivered;
        self.delivered = Some(when);
        self
    }
}