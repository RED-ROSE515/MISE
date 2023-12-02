use std::collections::HashSet;
use std::sync::Arc;

use eyre::Result;
use eyre::WrapErr;

use crate::cli::args::tool::{ToolArg, ToolArgParser};
use crate::config::Config;
use crate::output::Output;
use crate::runtime_symlinks;
use crate::shims;
use crate::tool::Tool;
use crate::toolset::{ToolVersion, ToolsetBuilder};
use crate::ui::multi_progress_report::MultiProgressReport;
use crate::ui::progress_report::ProgressReport;

/// Upgrades outdated tool versions
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct Upgrade {
    /// Tool(s) to upgrade
    /// e.g.: node@20 python@3.10
    /// If not specified, all current tools will be upgraded
    #[clap(value_name = "TOOL@VERSION", value_parser = ToolArgParser, verbatim_doc_comment)]
    pub tool: Vec<ToolArg>,
}

impl Upgrade {
    pub fn run(self, mut config: Config, _out: &mut Output) -> Result<()> {
        let mut ts = ToolsetBuilder::new()
            .with_args(&self.tool)
            .build(&mut config)?;
        let tool_set = self
            .tool
            .iter()
            .map(|t| t.plugin.clone())
            .collect::<HashSet<_>>();
        ts.versions
            .retain(|_, tvl| tool_set.is_empty() || tool_set.contains(&tvl.plugin_name));
        let outdated = ts.list_outdated_versions(&config);
        if outdated.is_empty() {
            info!("All tools are up to date");
        } else {
            self.upgrade(&mut config, outdated)?;
        }

        Ok(())
    }

    fn upgrade(&self, config: &mut Config, outdated: OutputVec) -> Result<()> {
        let mpr = MultiProgressReport::new(config.show_progress_bars());
        let mut ts = ToolsetBuilder::new().with_args(&self.tool).build(config)?;

        let new_versions = outdated
            .iter()
            .map(|(_, tv, latest)| {
                let mut tv = tv.clone();
                tv.version = latest.clone();
                tv
            })
            .collect();

        let to_remove = outdated
            .into_iter()
            .filter(|(tool, tv, _)| tool.is_version_installed(tv))
            .map(|(tool, tv, _)| (tool, tv))
            .collect::<Vec<_>>();

        ts.install_versions(config, new_versions, &mpr, false)?;
        for (tool, tv) in to_remove {
            let mut pr = mpr.add();
            self.uninstall_old_version(config, &tool, &tv, &mut pr)?;
        }

        let ts = ToolsetBuilder::new().with_args(&self.tool).build(config)?;
        shims::reshim(config, &ts).wrap_err("failed to reshim")?;
        runtime_symlinks::rebuild(config)?;
        Ok(())
    }

    fn uninstall_old_version(
        &self,
        config: &Config,
        tool: &Tool,
        tv: &ToolVersion,
        pr: &mut ProgressReport,
    ) -> Result<()> {
        tool.decorate_progress_bar(pr, Some(tv));
        match tool.uninstall_version(config, tv, pr, false) {
            Ok(_) => {
                pr.finish();
                Ok(())
            }
            Err(err) => {
                pr.error(err.to_string());
                Err(err.wrap_err(format!("failed to uninstall {tv}")))
            }
        }
    }
}

type OutputVec = Vec<(Arc<Tool>, ToolVersion, String)>;
