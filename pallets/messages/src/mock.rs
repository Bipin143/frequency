use crate as pallet_messages;
use common_primitives::msa::{
	AccountProvider, Delegator, KeyInfo, MessageSenderId, Provider, ProviderInfo,
};
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64, OnFinalize, OnInitialize},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchError,
};
use std::fmt::Formatter;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		MessagesPallet: pallet_messages::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxMessagesPerBlock: u32 = 500;
	pub const MaxMessageSizeInBytes: u32 = 100;
}

impl std::fmt::Debug for MaxMessageSizeInBytes {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MaxMessageSizeInBytes")
			.field("v", &MaxMessageSizeInBytes::get())
			.finish()
	}
}

impl PartialEq for MaxMessageSizeInBytes {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl Clone for MaxMessageSizeInBytes {
	fn clone(&self) -> Self {
		MaxMessageSizeInBytes {}
	}
}

pub struct AccountHandler;
impl AccountProvider for AccountHandler {
	type AccountId = u64;
	type BlockNumber = u64;
	fn get_msa_id(key: &Self::AccountId) -> Option<MessageSenderId> {
		if *key == 1000 {
			return None
		}
		if *key == 2000 {
			return Some(2000 as MessageSenderId)
		}
		Some(get_msa_from_account(*key) as MessageSenderId)
	}
	fn get_provider_info_of(
		provider: Provider,
		_delegator: Delegator,
	) -> Option<ProviderInfo<Self::BlockNumber>> {
		if provider == Provider(2000) {
			return None
		};
		Some(ProviderInfo { permission: 0, expired: 100 })
	}

	fn ensure_valid_msa_key(
		key: &Self::AccountId,
	) -> Result<KeyInfo<Self::BlockNumber>, DispatchError> {
		if *key == 1000 {
			return Err(DispatchError::Other("some error"))
		}
		if *key == 2000 {
			return Ok(KeyInfo { msa_id: 2000, nonce: 0, expired: 100 })
		}

		let info = KeyInfo { msa_id: get_msa_from_account(*key), nonce: 0, expired: 100 };
		Ok(info)
	}
}

impl pallet_messages::Config for Test {
	type Event = Event;
	type AccountProvider = AccountHandler;
	type WeightInfo = ();
	type MaxMessagesPerBlock = MaxMessagesPerBlock;
	type MaxMessageSizeInBytes = MaxMessageSizeInBytes;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			MessagesPallet::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		MessagesPallet::on_initialize(System::block_number());
	}
}

pub fn get_msa_from_account(account_id: u64) -> u64 {
	account_id + 100
}
