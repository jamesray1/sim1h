use crate::trace::LogContext;
use crate::trace::tracer;
use lib3h_protocol::data_types::StoreEntryAspectData;

/// Store data on a node's dht arc.
/// all entry aspects are in the database
/// no-op
pub fn handle_store_entry_aspect(log_context: &LogContext, store_entry_aspect_data: &StoreEntryAspectData) {
    tracer(&log_context, &format!("handle_store_entry_aspect {:?}", store_entry_aspect_data));
}
