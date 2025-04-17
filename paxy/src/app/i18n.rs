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

// region: TESTS

#[cfg(test)]
mod tests {
    use std::{env, fs};
    use fluent_fallback::{Localization, types::{ResourceType, ToResourceId}};
    use fluent_resmgr::ResourceManager;
    use unic_langid::LanguageIdentifier;
    use std::path::Path;

    fn check_file_exists(resource_path: &str, locale: &LanguageIdentifier, file_name: &str) {
        // Replace the {locale} placeholder with the actual locale string.
        let folder = resource_path.replace("{locale}", &locale.to_string());
        let full_path = Path::new(&folder).join(file_name);
        println!("Checking for file: {}", full_path.display());
        match fs::metadata(&full_path) {
            Ok(metadata) => println!("File exists (size {} bytes).", metadata.len()),
            Err(err) => println!("File check error: {}", err),
        }
    }

    #[test]
    fn hello_world_translation() {

        let lang_env = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        let lang_part = lang_env.split('.').next().unwrap();
        let locale_code = lang_part.replace('_', "-");
        let locale: LanguageIdentifier = locale_code.parse().unwrap_or_else(|_| "en".parse().unwrap());
        println!("Using locale: {}", locale);
        let resource_path = "./locales/{locale}/".to_owned();

        let file_name: &str = "messages.ftl";
        check_file_exists(&resource_path, &locale, file_name);

        let res_mgr = ResourceManager::new(resource_path);
        let sources = vec!["messages.ftl".into()];
        let mut localization = Localization::with_env(sources, true, vec![locale], res_mgr);
        let bundles = localization.bundles();
        let mut errors = Vec::new();

        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        println!("Hello");
        let translated = bundles.format_value_sync("hello-world", None, &mut errors).expect("Failed to format translation");
        println!("Hi");

        match translated {
            Some(text) => println!("Translation: {}", text),
            None => println!("Translation for 'hello-world' not found."),
        }

        match env::var("LANG") {
            Ok(val) if val == String::from("EN")  => {
                println!("lala");
            },
            Ok(val) if val == String::from("FR") => {
                println!("lulu");
            },
            _ => {
                println!("huhu");
            }
        }
    }
}

// endregion: TESTS