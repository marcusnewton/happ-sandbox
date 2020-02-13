#![feature(proc_macro_hygiene)]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::entry_definition::ValidatingEntryType;
use hdk::prelude::*;
use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.43-alpha3/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyEntry {
    content: String,
}

impl From<MyEntry> for Entry {
    fn from(entry: MyEntry) -> Entry {
        Entry::App("my_entry".into(), entry.into())
    }
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn my_entry_def() -> ValidatingEntryType {
        entry!(
            name: "my_entry",
            description: "this is an entry definition",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
        let entry = Entry::App("my_entry".into(), entry.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn update_my_entry(address: Address, entry: MyEntry) -> ZomeApiResult<Address> {
        hdk::update_entry(entry.into(), &address)
    }

    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn get_original_address(address: Address) -> ZomeApiResult<Address> {
        let get_entry_history = hdk::get_entry_history(&address)?;

        let original_address = match get_entry_history {
            Some(entry_history) => match entry_history.items[0].meta.clone() {
                Some(meta) => Ok(meta.address),
                None => Err(ZomeApiError::Internal("error".into())),
            },
            None => Err(ZomeApiError::Internal("error".into())),
        }?;

        Ok(original_address)
    }
}
