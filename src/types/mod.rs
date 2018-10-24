#[macro_use]
mod from_str;
mod address;
mod block;
mod blockchain;
mod keys;
mod script;
mod serde;
mod transaction;

#[derive(Deserialize, Serialize, Debug)]
pub struct Account(pub String);

#[allow(non_camel_case_types)]
// TODO: This enum is a bit weird. Clear it up once we have a better understanding of it
#[derive(Deserialize, Serialize, Debug)]
pub enum SigHashType {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "ALL|ANYONECANPAY")]
    All_AnyoneCanPay,
    #[serde(rename = "NONE|ANYONECANPAY")]
    None_AnyoneCanPay,
    #[serde(rename = "SINGLE|ANYONECANPAY")]
    Single_AnyoneCanPay,
}

pub use self::{address::*, block::*, blockchain::*, keys::*, script::*, transaction::*};
