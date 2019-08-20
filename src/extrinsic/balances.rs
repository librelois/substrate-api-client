/*
   Copyright 2019 Supercomputing Systems AG

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/

use codec::Compact;
use node_primitives::Hash;

use crate::{
    Api,
    compose_extrinsic,
    crypto::AccountKey,
    node_metadata::NodeMetadata
};

use super::xt_primitives::*;

pub const BALANCES_MODULE: &str = "Balances";
pub const BALANCES_TRANSFER: &str = "transfer";

pub type BalanceTransferFn = ([u8; 2], GenericAddress, Compact<u128>);

pub type BalanceTransferXt = UncheckedExtrinsicV3<BalanceTransferFn>;

pub fn transfer(api: Api, to: GenericAddress, amount: u128) -> BalanceTransferXt {
    compose_extrinsic!(
		api,
		BALANCES_MODULE,
		BALANCES_TRANSFER,
		to,
		Compact(amount)
	)
}