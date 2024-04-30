use std::env;
use std::path::PathBuf;

use swc_common::{chain, Mark};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use swc_ecma_visit::as_folder;
use swc_plugin_inline_env::transform::{inline_env, Config, Options};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    env::set_var("NODE_ENV", "production");
    env::set_var("WEBPACK_ENV", "false");

    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Default::default(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                as_folder(inline_env(
                    if input.to_string_lossy().contains("config") {
                        Config::WithOptions(Options {
                            only: vec!["NODE_ENV".into(), "NODE_ENV_NOT_EXIST".into()]
                        })
                } else {
                    Config::All(true)
                }))
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}