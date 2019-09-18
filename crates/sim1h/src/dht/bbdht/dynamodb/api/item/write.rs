use crate::dht::bbdht::dynamodb::client::Client;
use holochain_persistence_api::cas::content::AddressableContent;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use std::collections::HashMap;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::CONTENT_KEY;
use rusoto_dynamodb::PutItemError;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::PutItemOutput;
use holochain_persistence_api::cas::content::Address;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::log::trace;
use crate::log::LogContext;

pub fn ensure_content(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    content: &dyn AddressableContent,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    trace(&log_context, "ensure_content");
    let mut item = HashMap::new();
    item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(content.address())),
    );
    item.insert(
        String::from(CONTENT_KEY),
        string_attribute_value(&String::from(content.content())),
    );

    client
        .put_item(PutItemInput {
            item: item,
            table_name: table_name.to_string(),
            ..Default::default()
        })
        .sync()
}

pub fn touch_agent(log_context: &LogContext, client: &Client, table_name: &TableName, agent_id: &Address) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    trace(&log_context, "touch_agent");

    let mut item = HashMap::new();
    item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(agent_id.to_owned())),
    );
    client.put_item(PutItemInput {
        item: item,
        table_name: table_name.to_string(),
        ..Default::default()
    })
    .sync()
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::api::item::write::ensure_content;
    use crate::agent::fixture::agent_id_fresh;
    use crate::dht::bbdht::dynamodb::api::item::write::touch_agent;
    use crate::log::trace;

    #[test]
    fn ensure_content_test() {
        let log_context = "ensure_content_test";

        trace(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let content = content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).expect("could not check table exists"));

        // ensure content
        assert!(ensure_content(&log_context, &local_client, &table_name, &content).is_ok());

        // thrash a bit
        for _ in 0..100 {
            assert!(ensure_content(&log_context, &local_client, &table_name, &content).is_ok());
        }
    }

    #[test]
    fn touch_agent_test() {
        let log_context = "touch_agent_test";

        trace(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let agent_id = agent_id_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // touch agent
        assert!(touch_agent(&log_context, &local_client, &table_name, &agent_id).is_ok());
    }

}
