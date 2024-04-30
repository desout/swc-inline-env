#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use transform::{inline_env, Config};
pub mod transform;

#[plugin_transform]
fn swc_plugin(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Option<Config>>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for remove-console"),
    )
    .expect("invalid packages")
    .unwrap_or_else(|| Config::All(true));

    program.visit_mut_with(&mut inline_env(config
    ));

    program
}