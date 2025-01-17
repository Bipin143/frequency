#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use common_primitives::msa::*;
use frame_support::dispatch::DispatchError;
use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait MsaApi<AccountId> where
		AccountId: Codec,
	{
		fn get_msa_keys(msa_id: MessageSourceId) ->	Result<Vec<KeyInfoResponse<AccountId>>, DispatchError>;

		fn get_msa_id(key: AccountId) -> Result<Option<MessageSourceId>, DispatchError>;

		fn has_delegation(delegator: Delegator, provider: Provider) -> Result<bool, DispatchError>;
	}
}
