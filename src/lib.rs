#![doc = include_str!("../README.md")]

/// # Renewed Vision module/namespace
///
/// Contains majority of code related to ProPresenter
///
/// Originates from the `rv` and `registration.core` package within protobuf files
pub mod rv {
	/// # ProPresenter files and internal data
	///
	/// Contains structs representing data within ProPresenter
	pub mod data { include!(concat!(env!("OUT_DIR"), "/rv.data.rs")); }
	/// # ProPresenter analytics
	pub mod analytics { include!(concat!(env!("OUT_DIR"), "/rv.analytics.rs")); }
	/// # ProPresenter registration
	///
	/// Structs related to product registration
	pub mod registration { include!(concat!(env!("OUT_DIR"), "/registration.core.rs")); }
}