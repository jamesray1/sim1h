use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use lib3h_protocol::data_types::EntryAspectData;
use crate::dht::bbdht::dynamodb::schema::cas::PARTITION_KEY;
use holochain_persistence_api::cas::content::Address;
use crate::aspect::EntryAddress;
use crate::aspect::AspectAddress;
use crate::dht::bbdht::dynamodb::schema::cas::ITEM_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::NETWORK_KEY;
use crate::agent::AgentAddress;
use crate::dht::bbdht::dynamodb::schema::cas::SPACE_KEY;
use crate::space::Space;
use crate::network::RequestId;

pub mod fixture;
pub mod read;
pub mod write;

pub type Item = HashMap<String, AttributeValue>;

pub struct ItemKey(String);

impl From<&RequestId> for String {
    fn from(request_id: &RequestId) -> Self {
        request_id.to_owned().into()
    }
}

impl From<RequestId> for ItemKey {
    fn from(request_id: RequestId) -> Self {
        ItemKey(request_id.into())
    }
}

impl From<&RequestId> for ItemKey {
    fn from(request_id: &RequestId) -> Self {
        request_id.to_owned().into()
    }
}

impl From<String> for ItemKey {
    fn from(string: String) -> Self {
        ItemKey(string)
    }
}

impl From<ItemKey> for String {
    fn from(item_key: ItemKey) -> String {
        item_key.0
    }
}

impl From<&ItemKey> for String {
    fn from(item_key: &ItemKey) -> String {
        item_key.to_owned().into()
    }
}

impl From<Address> for ItemKey {
    fn from(address: Address) -> Self {
        ItemKey(address.into())
    }
}

impl From<AgentAddress> for ItemKey {
    fn from(agent_address: AgentAddress) -> Self {
        ItemKey(agent_address.into())
    }
}

impl From<&AgentAddress> for ItemKey {
    fn from(agent_address: &AgentAddress) -> Self {
        agent_address.to_owned().into()
    }
}

impl From<EntryAspectData> for ItemKey {
    fn from(entry_aspect_data: EntryAspectData) -> Self {
        entry_aspect_data.aspect_address.into()
    }
}

impl From<&EntryAspectData> for ItemKey {
    fn from(entry_aspect_data: &EntryAspectData) -> Self {
        entry_aspect_data.to_owned().into()
    }
}

impl From<AspectAddress> for ItemKey {
    fn from(aspect_address: AspectAddress) -> Self {
        ItemKey(aspect_address.into())
    }
}

impl From<&AspectAddress> for ItemKey {
    fn from(aspect_address: &AspectAddress) -> Self {
        aspect_address.to_owned().into()
    }
}

impl From<EntryAddress> for ItemKey {
    fn from(entry_address: EntryAddress) -> Self {
        ItemKey(entry_address.into())
    }
}

impl From<&EntryAddress> for ItemKey {
    fn from(entry_address: &EntryAddress) -> Self {
        entry_address.to_owned().into()
    }
}

fn partition_key(network: &String, space: &String, address: &String) -> String {
    format!("{}:{}:{}", network, space, address)
}

pub fn keyed_item(space: &Space, item_key: &ItemKey) -> Item {
    let mut item = HashMap::new();
    item.insert(
        String::from(PARTITION_KEY),
        string_attribute_value(
            &partition_key(
                &space.network_id.into(),
                &space.space_address.into(),
                &item_key.into(),
            )
        ),
    );
    item.insert(
        String::from(ITEM_KEY),
        string_attribute_value(
                &item_key.into(),
        ),
    );
    item.insert(
        String::from(NETWORK_KEY),
        string_attribute_value(
            &space.network_id.into(),
        ),
    );
    item.insert(
        String::from(SPACE_KEY),
        string_attribute_value(
            &space.space_address.into(),
        ),
    );
    item
}
