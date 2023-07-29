use serde::{Deserialize, Serialize};
use yew::Properties;

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
    IOFile {
        path: String,
    }
}
