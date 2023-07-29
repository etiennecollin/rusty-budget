use std::fmt::{Debug, self};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Arg {
    Nothing,
    Search {
        input: String,
    },
    GetTransactionCategories {
        kind: String,
    },
    AddTransaction {
        kind: String,
        category: String,
        description: String,
        date: String,
        amount: String,
        currency_symbol: String,
    },
    OpenProfile {
        path: String,
    },
    NewProfile {
        name: String,
    },
    AddAccount {
        description: String,
        currency_symbol: String,
    },
    SetCurrentAccountId {
        account_id: String,
    },
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, EnumString, Default)]
pub enum Page {
    #[default]
    Transactions,
    Accounts,
    Budget,
    Stats,
}
impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
