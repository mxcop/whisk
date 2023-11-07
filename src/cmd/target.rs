use owo_colors::OwoColorize;

use crate::man::{WhiskManifest, Target};

/// Merge two options into one.
fn merge<T>(base: &Option<T>, over: &Option<T>) -> Option<T>
where T: Clone {
    match (base, over) {
        (None, None) => None,
        (None, Some(over)) => Some(over.clone()),
        (Some(base), None) => Some(base.clone()),
        (Some(_), Some(over)) => Some(over.clone()),
    }
}

/// ### Build target
/// 
/// Get build target information for building the package.
pub fn get_target_info(man: &WhiskManifest, target_arg: Option<&String>, v: bool) -> Target {
    // Use the target argument, OR infer the target.
    let target = match target_arg {
        Some(target) => target.clone(),
        None => {
            #[cfg(target_pointer_width = "16")] {
                format!("{}-{}-16", std::env::consts::ARCH, std::env::consts::OS)
            }
            #[cfg(target_pointer_width = "32")] {
                format!("{}-{}-32", std::env::consts::ARCH, std::env::consts::OS)
            }
            #[cfg(target_pointer_width = "64")] {
                format!("{}-{}-64", std::env::consts::ARCH, std::env::consts::OS)
            }
        }
    };

    // Merge the super and specific target. (if one is present)
    let super_target = &man.package.target;
    match man.target {
        Some(ref targets) if targets.custom.contains_key(&target) => {
            let starget = &targets.custom[&target];

            // Verbose logging.
            if v {
                println!("   {} target = {} {}", "LOG".dimmed(), &target, "(overload found)".dimmed());
            }

            // Merge each option field.
            Target {
                compiler: merge(&super_target.compiler, &starget.compiler),
                src: merge(&super_target.src, &starget.src),
                include: merge(&super_target.include, &starget.include),
                libs: merge(&super_target.libs, &starget.libs),
                lib: merge(&super_target.lib, &starget.lib),
                link: merge(&super_target.link, &starget.link)
            }
        },
        _ => {
            // Verbose logging.
            if v {
                println!("   {} target = {} {}", "LOG".dimmed(), &target, "(no overload found)".dimmed());
            }

            super_target.clone()
        }
    }
}