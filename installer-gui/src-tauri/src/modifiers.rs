//! Adds or "modifies" installer CLI attributes for use in the GUI.
//!
//! This module contains little logic (outside of tests) and instead just provides additional
//! metadata about CLI commands and options for the GUI installer.
//!
//! If we like this approach, I think we should consider renaming this file something like
//! gui_modifiers.rs and moving it into the crate for the CLI installer. I think this would simplify
//! development as any breaking changes to the CLI installer interface would cause tests to fail in
//! its own crate instead of installer-gui and it'd help to keep the two interfaces to the installer
//! in sync.

#[derive(Debug, Copy, Clone)]
pub struct ArgumentModifier<'a> {
    /// The name of the argument on the CLI.
    pub cli_name: &'a str,
    /// The text for displaying this argument in the GUI.
    pub gui_label: &'a str,
    /// Whether this argument should be hidden behind a menu for "advanced" options.
    pub advanced: bool,
}

#[derive(Debug)]
pub struct SubcommandModifier<'a> {
    /// The name of the subcommand on the CLI.
    pub command: &'a str,
    /// The text for displaying this subcommand in the GUI.
    pub gui_label: &'a str,
    /// Modifications to the arguments of this subcommand. The order arguments are defined in this
    /// vector will match the order the arguments are displayed in the GUI.
    pub arg_modifiers: Vec<ArgumentModifier<'a>>,
}

/// Provides "modifiers" or additional metadata about each subcommand.
///
/// The order of the subcommands in the returned vector is the same order that subcommands will be
/// shown in the GUI.
pub fn subcommand_modifiers() -> Vec<SubcommandModifier<'static>> {
    // just for convenience, we define common ArgumentModifiers here that can be shared between
    // subcommands. if in the future the subcommands need slightly different settings for an
    // argument, the sharing of this code can be removed with no ill effects
    let admin_ip = ArgumentModifier {
        cli_name: "admin_ip",
        gui_label: "Admin IP",
        advanced: true,
    };
    let admin_username = ArgumentModifier {
        cli_name: "admin_username",
        gui_label: "Admin Username",
        advanced: true,
    };
    let admin_password = ArgumentModifier {
        cli_name: "admin_password",
        gui_label: "Admin Password",
        advanced: false,
    };
    let data_dir = ArgumentModifier {
        cli_name: "data_dir",
        gui_label: "Data Directory",
        advanced: true,
    };
    let reset_config = ArgumentModifier {
        cli_name: "reset_config",
        gui_label: "Reset config.toml",
        advanced: true,
    };
    let orbic_and_moxee_args = vec![
        admin_password,
        admin_ip,
        admin_username,
        reset_config,
        data_dir,
    ];

    vec![
        SubcommandModifier {
            command: "orbic",
            gui_label: "Orbic/Kajeet (via network)",
            arg_modifiers: orbic_and_moxee_args.clone(),
        },
        SubcommandModifier {
            command: "orbic-usb",
            gui_label: "Orbic/Kajeet (via legacy USB+ADB installer)",
            arg_modifiers: vec![reset_config],
        },
        SubcommandModifier {
            command: "tplink",
            gui_label: "TP-Link",
            arg_modifiers: vec![
                admin_ip,
                reset_config,
                data_dir,
                ArgumentModifier {
                    cli_name: "skip_sdcard",
                    gui_label: "Skip SD Card",
                    advanced: true,
                },
                ArgumentModifier {
                    cli_name: "sdcard_path",
                    gui_label: "SD Card Path",
                    advanced: true,
                },
            ],
        },
        SubcommandModifier {
            command: "moxee",
            gui_label: "Moxee",
            arg_modifiers: orbic_and_moxee_args,
        },
        SubcommandModifier {
            command: "pinephone",
            gui_label: "PinePhone",
            arg_modifiers: vec![],
        },
        SubcommandModifier {
            command: "tmobile",
            gui_label: "TMobile",
            arg_modifiers: vec![admin_password, admin_ip],
        },
        SubcommandModifier {
            command: "uz801",
            gui_label: "UZ801",
            arg_modifiers: vec![admin_ip],
        },
        SubcommandModifier {
            command: "wingtech",
            gui_label: "Wingtech",
            arg_modifiers: vec![admin_password, admin_ip],
        },
    ]
}

#[cfg(test)]
mod tests {
    //! The GUI code is written to simply not include any subcommand or argument that doesn't have a
    //! modifier defined for it. To avoid us unintentionally excluding items, the tests below ensure
    //! every CLI option either has a modifier or is explicitly excluded from the GUI.
    use super::*;
    use std::collections::HashMap;

    fn excluded_arguments() -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::new()
    }

    fn excluded_subcommands() -> Vec<&'static str> {
        vec!["util"]
    }

    #[test]
    fn test_subcommands() {
        let exclusions = excluded_subcommands();

        let mut clap_subcommands: Vec<&str> = crate::INSTALLER_COMMAND
            .get_subcommands()
            .filter_map(|c| {
                let name = c.get_name();
                if exclusions.contains(&name) {
                    None
                } else {
                    Some(name)
                }
            })
            .collect();
        let mut modified_subcommands: Vec<&str> = subcommand_modifiers()
            .into_iter()
            .map(|m| m.command)
            .collect();

        clap_subcommands.sort_unstable();
        modified_subcommands.sort_unstable();

        assert_eq!(
            clap_subcommands, modified_subcommands,
            "Every subcommand must be included exactly once in subcommand_modifiers() or excluded_subcommands()."
        );
    }

    #[test]
    fn test_arguments() {
        let exclusions = excluded_subcommands();

        // create vectors of (subcommand, argument_name) tuples
        let mut clap_args: Vec<(&str, &str)> = crate::INSTALLER_COMMAND
            .get_subcommands()
            .filter_map(|c| {
                let subcommand_name = c.get_name();
                if exclusions.contains(&subcommand_name) {
                    None
                } else {
                    Some(std::iter::zip(
                        std::iter::repeat(subcommand_name),
                        c.get_arguments().map(|a| a.get_id().as_str()),
                    ))
                }
            })
            .flatten()
            .collect();
        let mut modified_args: Vec<(&str, &str)> = subcommand_modifiers()
            .into_iter()
            .flat_map(|m| {
                std::iter::zip(
                    std::iter::repeat(m.command),
                    m.arg_modifiers.into_iter().map(|arg_m| arg_m.cli_name),
                )
            })
            .collect();

        clap_args.sort_unstable();
        modified_args.sort_unstable();

        // if in the future we want to exclude individual arguments from a subcommand, we can adopt
        // an approach similar to excluded_subcommands above for excluded arguments
        assert_eq!(
            clap_args, modified_args,
            "Every argument for non-excluded subcommands must have exactly one ArgumentModifier."
        );
    }
}
