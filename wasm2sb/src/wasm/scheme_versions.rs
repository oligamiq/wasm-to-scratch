use semver::Version;

pub fn scheme_versions() -> Vec<Version> {
    let mut versions = vec![
        Version::parse("0.0.0").unwrap(),
        Version::parse("0.1.0").unwrap(),
    ];
    versions.sort();
    versions
}
