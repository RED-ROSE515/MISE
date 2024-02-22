use eyre::Result;

/// Output direnv function to use mise inside direnv
///
/// See https://mise.jdx.dev/direnv.html for more information
///
/// Because this generates the legacy files based on currently installed plugins,
/// you should run this command after installing new plugins. Otherwise
/// direnv may not know to update environment variables when legacy file versions change.
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment, after_long_help = AFTER_LONG_HELP)]
pub struct DirenvActivate {}

impl DirenvActivate {
    pub fn run(self) -> Result<()> {
        miseprintln!(
            //       source_env "$(mise direnv envrc "$@")"
            indoc! {r#"
                ### Do not edit. This was autogenerated by 'mise direnv' ###
                use_mise() {{
                  direnv_load mise direnv exec
                }}
            "#}
        );

        Ok(())
    }
}

static AFTER_LONG_HELP: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</underline></bold>

    $ <bold>mise direnv activate > ~/.config/direnv/lib/use_mise.sh</bold>
    $ <bold>echo 'use mise' > .envrc</bold>
    $ <bold>direnv allow</bold>
"#
);
