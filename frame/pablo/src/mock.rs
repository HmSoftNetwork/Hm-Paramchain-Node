#![cfg(test)]

use crate as pablo;
use frame_support::{parameter_types, traits::Everything, PalletId};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_arithmetic::traits::Zero;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	Permill,
};
use system::EnsureRoot;

pub type CurrencyId = u128;
pub type BlockNumber = u64;
pub type Moment = composable_traits::time::Timestamp;

pub const BTC: AssetId = 0;
pub const USDT: CurrencyId = 2;
pub const USDC: CurrencyId = 4;
pub const PROJECT_TOKEN: AssetId = 1;
pub const TWAP_INTERVAL: Moment = 10;
pub const MILLISECS_PER_BLOCK: u64 = 12000;

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
		Pablo: pablo::{Pallet, Call, Storage, Event<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
	}
);

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRoot<AccountId>;
	type ReserveOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

pub type AccountId = u128;

#[allow(dead_code)]
pub static ALICE: AccountId = 1;
#[allow(dead_code)]
pub static BOB: AccountId = 2;
#[allow(dead_code)]
pub static CHARLIE: AccountId = 3;
#[allow(dead_code)]
pub static CURVE_ADMIN_FEE_ACC_ID: AccountId = 4;

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type Balance = u128;
pub type AssetId = u128;
pub type Amount = i128;
pub type PoolId = u128;

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

parameter_types! {
	pub Precision: u128 = 100_u128;
	pub TestPalletID : PalletId = PalletId(*b"pablo_pa");
	pub MinSaleDuration: BlockNumber = 3600 / 12;
	pub MaxSaleDuration: BlockNumber = 30 * 24 * 3600 / 12;
	pub MaxInitialWeight: Permill = Permill::from_percent(95);
	pub MinFinalWeight: Permill = Permill::from_percent(5);
	pub const TWAPInterval: Moment = MILLISECS_PER_BLOCK * TWAP_INTERVAL;
}

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pablo::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type CurrencyFactory = LpTokenFactory;
	type Assets = Tokens;
	type Convert = ConvertInto;
	type PoolId = PoolId;
	type PalletId = TestPalletID;
	type LocalAssets = LpTokenFactory;
	type LbpMinSaleDuration = MinSaleDuration;
	type LbpMaxSaleDuration = MaxSaleDuration;
	type LbpMaxInitialWeight = MaxInitialWeight;
	type LbpMinFinalWeight = MinFinalWeight;
	type PoolCreationOrigin = EnsureRoot<AccountId>;
	type EnableTwapOrigin = EnsureRoot<AccountId>;
	type Time = Timestamp;
	type TWAPInterval = TWAPInterval;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
