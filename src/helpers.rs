use std::vec;

use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_hugo() -> Result<String, Error> {
    let os = dag().get_os()?;
    let arch = dag().get_arch()?;

    match arch.as_str() {
        "x86_64" => {
            dag().set_envs(vec![("ARCH".into(), "amd64".into())])?;
        }
        "aarch64" => {
            dag().set_envs(vec![("ARCH".into(), "arm64".into())])?;
        }
        _ => {
            dag().set_envs(vec![("ARCH".into(), arch)])?;
        }
    }

    match os.as_str() {
        "macos" => {
            dag().set_envs(vec![
                ("OS".into(), "darwin".into()),
                ("ARCH".into(), "universal".into()),
            ])?;
        }
        "linux" => {
            dag().set_envs(vec![("OS".into(), "linux".into())])?;
        }
        _ => {
            dag().set_envs(vec![("OS".into(), os)])?;
        }
    }
    let stdout = dag().pkgx()?
      .with_workdir("/tmp")?
      .with_exec(vec!["type hugo > /dev/null 2> /dev/null || pkgx wget https://github.com/gohugoio/hugo/releases/download/v${HUGO_VERSION}/hugo_${HUGO_VERSION}_${OS}-${ARCH}.tar.gz"])?
      .with_exec(vec!["type hugo > /dev/null 2> /dev/null || pkgx tar -xzf hugo_${HUGO_VERSION}_${OS}-${ARCH}.tar.gz"])?
      .with_exec(vec!["type hugo > /dev/null 2> /dev/null || mv hugo $HOME/.local/bin"])?
      .with_exec(vec!["rm -rf hugo* README.md LICENSE"])?
      .stdout()?;

    Ok(stdout)
}
