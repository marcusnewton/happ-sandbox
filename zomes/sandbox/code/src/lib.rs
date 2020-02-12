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
pub struct MyParentEntry {
    content: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyChildEntry {
    content: String,
}

impl From<MyParentEntry> for Entry {
    fn from(entry: MyParentEntry) -> Entry {
        Entry::App("my_parent_entry".into(), entry.into())
    }
}

impl From<MyChildEntry> for Entry {
    fn from(entry: MyChildEntry) -> Entry {
        Entry::App("my_child_entry".into(), entry.into())
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
    fn my_parent_entry_def() -> ValidatingEntryType {
        entry!(
            name: "my_parent_entry",
            description: "this is a parent entry definition",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MyParentEntry>| {
                Ok(())
            },
            links: [
                to!(
                    "my_child_entry",
                    link_type: "parent_to_child",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[entry_def]
    fn my_child_entry_def() -> ValidatingEntryType {
        entry!(
            name: "my_child_entry",
            description: "this is a child entry definition",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MyChildEntry>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_my_parent_entry(entry: MyParentEntry) -> ZomeApiResult<Address> {
        let entry = Entry::App("my_parent_entry".into(), entry.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn create_my_child_entry(parent: Address, entry: MyChildEntry) -> ZomeApiResult<Address> {
        let entry = Entry::App("my_child_entry".into(), entry.into());
        let address = hdk::commit_entry(&entry)?;
        let _link = hdk::link_entries(&parent, &address, "parent_to_child", "")?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn update_my_parent_entry(address: Address, entry: MyParentEntry) -> ZomeApiResult<Address> {
        hdk::update_entry(entry.into(), &address)
    }

    #[zome_fn("hc_public")]
    fn update_my_child_entry(address: Address, entry: MyChildEntry) -> ZomeApiResult<Address> {
        hdk::update_entry(entry.into(), &address)
    }

    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn get_children(address: Address) -> ZomeApiResult<GetLinksResult> {
        let get_entry_result = hdk::get_entry_result(
            &address,
            GetEntryOptions {
                status_request: StatusRequestKind::Initial,
                entry: false,
                headers: true,
                timeout: Default::default(),
            },
        )?;

        let headers = match get_entry_result.result {
            GetEntryResultType::Single(item) => Ok(item.headers),
            _ => Err(ZomeApiError::Internal(
                "Error getting headers of parent entry".into(),
            )),
        }?;

        let canonical_address = headers[0].entry_address();

        hdk::get_links(
            &canonical_address,
            LinkMatch::Exactly("parent_to_child"),
            LinkMatch::Any,
        )
    }
}
