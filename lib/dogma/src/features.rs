// This is free and unencumbered software released into the public domain.

// Keep one list for the cfg-gated API and the manifest consistency check.
macro_rules! declare_features {
    ($($name:literal),* $(,)?) => {
        /// The enabled Cargo features of this crate, in alphabetical order.
        ///
        /// Includes allocation, integrations, and umbrella flags such as `all`
        /// and `default` when enabled. Flags are reported as selected by Cargo,
        /// including transitive activation, rather than inferred from APIs.
        pub static FEATURES: &[&str] = &[$(#[cfg(feature = $name)] $name),*];

        #[cfg(test)]
        const DECLARED_FEATURES: &[&str] = &[$($name),*];
    };
}

declare_features!(
    "all",
    "alloc",
    "camino",
    "clap",
    "collection",
    "countable",
    "default",
    "enums",
    "iri",
    "labeled",
    "miette",
    "named",
    "serde",
    "std",
    "structs",
    "traits",
    "unstable",
    "uri",
);

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::{DECLARED_FEATURES, FEATURES};
    use alloc::vec::Vec;

    #[test]
    fn feature_inventory_matches_manifest() {
        // The manifest uses unquoted, single-line keys in [features]. Values
        // may span lines; only lines containing an assignment declare a key.
        let mut names: Vec<_> = include_str!("../Cargo.toml")
            .lines()
            .map(str::trim)
            .skip_while(|line| *line != "[features]")
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .filter(|line| !line.starts_with('#'))
            .filter_map(|line| line.split_once('=').map(|(name, _)| name.trim()))
            .collect();
        names.sort_unstable();
        assert_eq!(DECLARED_FEATURES, names);
    }

    #[test]
    fn enabled_features_are_sorted_and_unique() {
        assert!(FEATURES.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(FEATURES.iter().all(|name| DECLARED_FEATURES.contains(name)));
    }
}
