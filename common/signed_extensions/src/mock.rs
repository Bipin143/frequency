use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU32, ConstU64, EqualPrivilegeOnly, Get},
};
use frame_system as system;
use frame_system::{EnsureRoot, EnsureSignedBy};
use pallet_balances;
use pallet_democracy;
pub use pallet_democracy::Call as DemocracyCall;

use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, SignedExtension}, MultiSignature,
};

pub type BlockNumber = u64;
pub type AccountId = u64;
pub type Block = frame_system::mocking::MockBlock<Test>;
pub type Signature = MultiSignature;

// pub type SignedExtra = signed_extensions::democracy::VerifyVoter<Test>;
// pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<AccountId, Call, Signature, SignedExtra>;
//pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, u64, Call, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		Preimage: pallet_preimage,
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>},
		Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

impl system::Config for Test {
	type AccountId = AccountId;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const MILLISECS_PER_BLOCK: u64 = 12000;

ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
}

pub type Balance = u64;
pub const UNIT: Balance = 1_000_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

parameter_types! {
	pub const EnactmentPeriod: BlockNumber = 30 * 24 * 60 * MINUTES;
	pub const LaunchPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const PreimageByteDeposit: Balance = 1 * MICROUNIT;
	pub const VotingPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const MinimumDeposit: Balance = 100 * UNIT;
}

impl pallet_democracy::Config for Test {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type MinimumDeposit = MinimumDeposit;
	type PreimageByteDeposit = PreimageByteDeposit;
	type CooloffPeriod = VotingPeriod;
	type FastTrackVotingPeriod = VotingPeriod;
	type VoteLockingPeriod = VotingPeriod;
	type VotingPeriod = VotingPeriod;
	type MaxVotes = ConstU32<100>;
	type MaxProposals = ConstU32<100>;
	type ExternalOrigin = EnsureSignedBy<Two, u64>;
	type ExternalMajorityOrigin = EnsureSignedBy<Two, u64>;
	type ExternalDefaultOrigin = EnsureSignedBy<Two, u64>;
	type FastTrackOrigin = EnsureSignedBy<Two, u64>;
	type CancellationOrigin = EnsureSignedBy<Two, u64>;
	type BlacklistOrigin = EnsureRoot<u64>;
	type CancelProposalOrigin = EnsureRoot<u64>;
	type VetoOrigin = EnsureSignedBy<Two, u64>;
	type Slash = ();
	type InstantOrigin = EnsureSignedBy<Two, u64>;
	type InstantAllowed = frame_support::traits::ConstBool<true>;
	type Scheduler = Scheduler;
	type OperationalPreimageOrigin = EnsureSignedBy<Two, u64>;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

impl pallet_preimage::Config for Test {
	type Event = Event;
	type WeightInfo = ();
	type Currency = ();
	type ManagerOrigin = EnsureRoot<u64>;
	type MaxSize = ConstU32<1024>;
	type BaseDeposit = ();
	type ByteDeposit = ();
}

impl pallet_scheduler::Config for Test {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = ConstU64<2_000_000_000_000>;
	type ScheduleOrigin = EnsureRoot<u64>;
	type MaxScheduledPerBlock = ConstU32<100>;
	type WeightInfo = ();
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = ConstU64<10>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<10>;
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1_000_000_000>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	let mut ext: sp_io::TestExternalities = GenesisConfig {
// 		collective: pallet_collective::GenesisConfig {
// 			members: vec![1, 2, 3],
// 			phantom: Default::default(),
// 		},
// 		default_collective: Default::default(),
// 	}
// 		.build_storage()
// 		.unwrap()
// 		.into();
// 	ext.execute_with(|| System::set_block_number(1));
// 	ext
// }

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
