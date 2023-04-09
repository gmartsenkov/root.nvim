use std::path::Path;

use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    patterns: Vec<String>,
}

fn find_root(file: String, config: &Config) -> Option<String> {
    let path = Path::new(&file);
    let mut ancestors = path.ancestors();

    while let Some(dir) = ancestors.next() {
        for pattern in &config.patterns {
            let mut pathbuf = dir.to_path_buf();
            pathbuf.push(pattern);
            if pathbuf.exists() {
                return Some(dir.to_str().unwrap().to_string());
            }
        }
    }
    None
}

fn lua_find(lua: &Lua, (file, config): (String, LuaValue)) -> LuaResult<Option<String>> {
    let config: Config = lua.from_value(config).unwrap();

    match find_root(file, &config) {
        Some(dir) => return Ok(Some(dir)),
        None => return Ok(None),
    }
}

#[mlua::lua_module]
fn root_backend(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("find", lua.create_function(lua_find)?)?;
    Ok(exports)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        file: &'static str,
        expected: Option<&'static str>,
        config: Config,
    }

    #[test]
    fn test_find_root() {
        let config = Config {
            patterns: vec!["Gemfile".to_string()],
        };

        let tests = vec![
            Test {
                file: "./tests/fixtures/dev/lib/api.rb",
                expected: Some("./tests/fixtures/dev/lib"),
                config: config.clone(),
            },
            Test {
                file: "./tests/fixtures/dev/lib/endpoints/user.rb",
                expected: Some("./tests/fixtures/dev/lib"),
                config: config.clone(),
            },
            Test {
                file: "./tests/fixtures/dev/app/web.rb",
                expected: None,
                config: config.clone(),
            },
            Test {
                file: "./tests/fixtures/dev/app/web.rb",
                expected: Some("./tests/fixtures/dev"),
                config: Config { patterns: vec!["app".to_string()] }
            },
        ];

        for test in tests {
            assert_eq!(
                find_root(test.file.to_string(), &test.config),
                test.expected.map(|f| f.to_string())
            )
        }
    }
}
