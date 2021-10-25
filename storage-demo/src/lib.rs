use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, Vector};
use near_sdk::json_types::{ValidAccountId};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BorshStorageKey,PanicOnDefault, 
};

use note::{NoteBook, TextNoteBook, RichNoteBook};

mod note;


#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Notes,
    RichNotes { notes_id: u32 },
    Friends,
}


near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub notes: Vector<NoteBook>,
    pub friends: UnorderedSet<AccountId>,
    pub total_balance: Balance,
    pub friend_rate: HashMap<AccountId, u32>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
    ) -> Self {
        Self {
            owner_id: owner_id.into(),
            notes: Vector::new(StorageKey::Notes),
            friends: UnorderedSet::new(StorageKey::Friends),
            total_balance: 0,
            friend_rate: HashMap::new(),
        }
    }

    pub fn add_text_notebook(&mut self) -> u32 {
        let id = self.notes.len() as u32;
        let notebook = NoteBook::TextNoteBook(TextNoteBook::new(id));   
        self.notes.push(&notebook);
        id
    }

    pub fn add_rich_notebook(&mut self) -> u32 {
        let id = self.notes.len() as u32;
        let notebook = NoteBook::RichNoteBook(RichNoteBook::new(id));   
        self.notes.push(&notebook);
        id
    }

    pub fn add_notes(&mut self, notebook_id: u32, content: String) {
        let sender_id = env::predecessor_account_id();
        let mut notebook = self.notes.get(notebook_id.into()).expect("ERR_NO_NOTEBOOK");
        notebook.add_notes(&sender_id, &content);
        self.notes.replace(notebook_id.into(), &notebook);
    }

    pub fn add_friends(&mut self, friends: Vec<ValidAccountId>) {
        for friend in friends {
            self.friends.insert(friend.as_ref());
            self.friend_rate.insert(friend.into(), 0);
        }
    }

    pub fn remove_friends(&mut self, friends: Vec<ValidAccountId>) {
        for friend in friends {
            self.friends.remove(friend.as_ref());
        }
    }
}

