use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;
use helpers::setup_hugo;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "0.131.0".into()
    } else {
        version
    };

    dag().set_envs(vec![("HUGO_VERSION".into(), version)])?;

    let stdout = setup_hugo()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    setup_hugo()?;
    let stdout = dag()
        .pipeline("build")?
        .with_exec(vec!["hugo", &args])?
        .stdout()?;
    Ok(stdout)
}
