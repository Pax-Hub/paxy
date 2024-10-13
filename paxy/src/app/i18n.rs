//! Translation/Internationalization/Regionalization.

pub fn init_i18n(
    config: &config::ConfigTemplate
) -> Result<fluent_fallback::Bundles, Error> {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/".to_string());
    Ok(
        fluent_fallback::Localization::with_env(vec![
            "test.ftl".into(),
            "test2.ftl".to_resource_id(ResourceType::Optional),
        ],
        true,
        vec![langid!("en-US")],
        ResourceManager::new("/tmp/paxy/{locale}/".to_string())).bundles()
    )
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    I18nDummy {},
}

// endregion: ERRORS

// region: IMPORTS

use fluent_fallback::Localization;
use snafu::Snafu;

// endregion: IMPORTS
