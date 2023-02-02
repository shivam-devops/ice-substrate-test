
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-10-175`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ice-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_identity
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/frost/src/weights/pallet_identity_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 27_646 nanoseconds.
		Weight::from_ref_time(29_420_887)
			// Standard Error: 4_807
			.saturating_add(Weight::from_ref_time(287_539).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 65_107 nanoseconds.
		Weight::from_ref_time(60_607_721)
			// Standard Error: 5_008
			.saturating_add(Weight::from_ref_time(327_219).saturating_mul(r.into()))
			// Standard Error: 977
			.saturating_add(Weight::from_ref_time(740_085).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:2 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 17_938 nanoseconds.
		Weight::from_ref_time(48_483_716)
			// Standard Error: 7_024
			.saturating_add(Weight::from_ref_time(4_650_757).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:2)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 17_682 nanoseconds.
		Weight::from_ref_time(48_810_966)
			// Standard Error: 7_187
			.saturating_add(Weight::from_ref_time(1_973_696).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 100_025 nanoseconds.
		Weight::from_ref_time(65_599_406)
			// Standard Error: 11_736
			.saturating_add(Weight::from_ref_time(74_954).saturating_mul(r.into()))
			// Standard Error: 2_291
			.saturating_add(Weight::from_ref_time(1_883_298).saturating_mul(s.into()))
			// Standard Error: 2_291
			.saturating_add(Weight::from_ref_time(385_219).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 65_340 nanoseconds.
		Weight::from_ref_time(61_356_951)
			// Standard Error: 6_053
			.saturating_add(Weight::from_ref_time(328_504).saturating_mul(r.into()))
			// Standard Error: 1_181
			.saturating_add(Weight::from_ref_time(764_087).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 60_354 nanoseconds.
		Weight::from_ref_time(56_662_252)
			// Standard Error: 5_737
			.saturating_add(Weight::from_ref_time(283_172).saturating_mul(r.into()))
			// Standard Error: 1_119
			.saturating_add(Weight::from_ref_time(768_331).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 15_172 nanoseconds.
		Weight::from_ref_time(15_948_129)
			// Standard Error: 3_033
			.saturating_add(Weight::from_ref_time(211_546).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 15_206 nanoseconds.
		Weight::from_ref_time(16_265_204)
			// Standard Error: 3_640
			.saturating_add(Weight::from_ref_time(211_291).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 14_928 nanoseconds.
		Weight::from_ref_time(15_839_666)
			// Standard Error: 3_073
			.saturating_add(Weight::from_ref_time(213_183).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 48_688 nanoseconds.
		Weight::from_ref_time(46_005_640)
			// Standard Error: 7_590
			.saturating_add(Weight::from_ref_time(280_073).saturating_mul(r.into()))
			// Standard Error: 1_404
			.saturating_add(Weight::from_ref_time(1_231_941).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 121_264 nanoseconds.
		Weight::from_ref_time(79_764_369)
			// Standard Error: 19_575
			.saturating_add(Weight::from_ref_time(293_367).saturating_mul(r.into()))
			// Standard Error: 3_822
			.saturating_add(Weight::from_ref_time(1_947_874).saturating_mul(s.into()))
			// Standard Error: 3_822
			.saturating_add(Weight::from_ref_time(411_008).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 57_505 nanoseconds.
		Weight::from_ref_time(63_928_727)
			// Standard Error: 2_341
			.saturating_add(Weight::from_ref_time(217_811).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 23_485 nanoseconds.
		Weight::from_ref_time(26_394_890)
			// Standard Error: 1_187
			.saturating_add(Weight::from_ref_time(112_756).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 62_117 nanoseconds.
		Weight::from_ref_time(66_359_082)
			// Standard Error: 1_690
			.saturating_add(Weight::from_ref_time(192_670).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 42_902 nanoseconds.
		Weight::from_ref_time(47_375_048)
			// Standard Error: 1_671
			.saturating_add(Weight::from_ref_time(184_216).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}