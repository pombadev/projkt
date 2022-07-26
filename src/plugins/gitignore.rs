use std::{fs::OpenOptions, path::PathBuf, sync::Arc};

use skim::SkimItem;

use crate::plugins::{
    types::{Plugin, ProjktResult},
    utils::{fuzzy, write_or_create, FuzzyItemType},
};

mod fetcher {
    const SECS_IN_WEEK: u64 = 60 * 60 * 24 * 7;

    use std::{
        env,
        fs::{self, File},
        path::{Path, PathBuf},
        time::SystemTime,
    };

    use crate::plugins::{types::ProjktResult, utils::FuzzyItemType};

    use ureq::serde_json;

    fn cache_path() -> PathBuf {
        Path::new(&dirs::cache_dir().unwrap())
            .join(env!("CARGO_PKG_NAME"))
            .join("gitignore.json")
    }

    fn parse(val: &serde_json::Value) -> Vec<FuzzyItemType> {
        val.as_object()
            .expect("response is expected to be a json object")
            .iter()
            .filter_map(|(key, item)| {
                item["contents"]
                    .as_str()
                    .map(|contents| FuzzyItemType(key.to_string(), contents.into()))
            })
            .collect()
    }

    fn fetch_from_remote() -> ProjktResult<serde_json::Value> {
        let res = ureq::get("https://www.toptal.com/developers/gitignore/api/list?format=json")
            .call()?
            .into_json::<serde_json::Value>()?;

        Ok(res)
    }

    fn fetch_from_cache() -> ProjktResult<serde_json::Value> {
        let res = serde_json::from_reader(File::open(cache_path())?)?;

        Ok(res)
    }

    fn cache_expired() -> ProjktResult<bool> {
        let file = cache_path();

        if file.exists() {
            let now = SystemTime::now();
            let created = file.metadata()?.created()?;
            let diff = now.duration_since(created)?.as_secs();

            if diff < SECS_IN_WEEK {
                Ok(false)
            } else {
                Ok(true)
            }
        } else {
            Ok(true)
        }
    }

    fn save_to_cache(val: &serde_json::Value) -> ProjktResult<()> {
        let full_path = cache_path();
        let dir = full_path.parent().unwrap();

        fs::create_dir_all(dir)?;

        let file = File::create(full_path)?;

        serde_json::to_writer(file, val)?;

        Ok(())
    }

    pub fn get() -> ProjktResult<Vec<FuzzyItemType>> {
        let resource = if cache_expired()? {
            let response = fetch_from_remote()?;
            save_to_cache(&response)?;
            response
        } else {
            fetch_from_cache()?
        };

        Ok(parse(&resource))
    }
}

pub struct GitIgnoreOptions {
    pub append: bool,
    pub dest: PathBuf,
    pub name: Option<String>,
    pub overwrite: bool,
}

pub struct GitIgnore;

impl GitIgnore {
    fn write(opts: &GitIgnoreOptions, data: &[Arc<dyn SkimItem>]) -> ProjktResult<()> {
        let dotgitignore = PathBuf::from(&opts.dest).join(".gitignore");

        let contents = data
            .iter()
            .flat_map(|item| {
                let ret: Box<[u8]> = Box::from(item.output().as_bytes());
                Box::leak(ret)
            })
            .map(|item| *item)
            .collect::<Vec<_>>();

        write_or_create(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(opts.overwrite)
                .append(opts.append),
            &dotgitignore,
            contents.as_slice(),
            opts.overwrite,
        )?;

        Ok(())
    }
}

impl Plugin for GitIgnore {
    type Opts = GitIgnoreOptions;
    type Fetch = Vec<FuzzyItemType>;

    fn fetch(_: &Self::Opts) -> ProjktResult<Self::Fetch> {
        fetcher::get()
    }

    fn exec(opts: Self::Opts) -> ProjktResult<()> {
        match opts.name {
            Some(ref name) => {
                let items = Self::fetch(&opts)?;

                match items.iter().find(|item| &item.0 == name) {
                    Some(template) => {
                        Self::write(&opts, &[Arc::new(template.1.clone())])?;
                        Ok(())
                    }

                    None => {
                        let templates = items
                            .iter()
                            .map(|item| item.0.clone())
                            .collect::<Vec<String>>()
                            .join(", ");

                        return Err(format!(
                            "gitignore template for `{name}` not found, available:\n {templates}"
                        )
                        .into());
                    }
                }
            }
            None => {
                let items = Self::fetch(&opts)?;
                let selected = fuzzy(items, true)?;

                Self::write(&opts, &selected)?;

                Ok(())
            }
        }
    }
}
