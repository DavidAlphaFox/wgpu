use pico_args::Arguments;
use xshell::Shell;

#[derive(Debug)]
enum Search<'a> {
    #[expect(dead_code)]
    Positive(&'a str),
    Negative(&'a str),
}

#[derive(Debug)]
struct Requirement<'a> {
    human_readable_name: &'a str,
    target: &'a str,
    packages: &'a [&'a str],
    features: &'a [&'a str],
    default_features: bool,
    search_terms: &'a [Search<'a>],
}

const ALL_WGPU_FEATURES: &[&str] = &[
    "dx12",
    "metal",
    "webgpu",
    "angle",
    "vulkan-portability",
    "webgl",
    "spirv",
    "glsl",
    "wgsl",
    "naga-ir",
    "serde",
    "replay",
    "counters",
    "fragile-send-sync-non-atomic-wasm",
    "static-dxc",
];

pub fn check_feature_dependencies(shell: Shell, arguments: Arguments) -> anyhow::Result<()> {
    let mut _args = arguments.finish();

    let features_no_webgl: Vec<&str> = ALL_WGPU_FEATURES
        .iter()
        .copied()
        .filter(|feature| *feature != "webgl")
        .collect();

    let requirements = [
        Requirement {
            human_readable_name: "wasm32 without `webgl` feature does not depend on `wgpu-core`",
            target: "wasm32-unknown-unknown",
            packages: &["wgpu"],
            features: &features_no_webgl,
            default_features: false,
            search_terms: &[Search::Negative("wgpu-core")],
        },
        Requirement {
            human_readable_name:
                "wasm32 with `webgpu` and `wgsl` feature does not depend on `naga`",
            target: "wasm32-unknown-unknown",
            packages: &["wgpu"],
            features: &["webgpu", "wgsl"],
            default_features: false,
            search_terms: &[Search::Negative("naga")],
        },
    ];

    let mut any_failures = false;
    for requirement in requirements {
        let mut cmd = shell
            .cmd("cargo")
            .args(["tree", "--target", requirement.target]);

        for package in requirement.packages {
            cmd = cmd.arg("--package").arg(package);
        }

        if !requirement.default_features {
            cmd = cmd.arg("--no-default-features");
        }

        if !requirement.features.is_empty() {
            cmd = cmd.arg("--features").arg(requirement.features.join(","));
        }

        log::info!("Checking Requirement: {}", requirement.human_readable_name);
        log::debug!("{:#?}", requirement);
        log::debug!("$ {cmd}");

        let output = cmd.read()?;

        log::debug!("{output}");

        for search_term in requirement.search_terms {
            let found = match search_term {
                Search::Positive(search_term) => output.contains(search_term),
                Search::Negative(search_term) => !output.contains(search_term),
            };

            if found {
                log::info!("✅ Passed!");
            } else {
                log::info!("❌ Failed");
                any_failures = true;
            }
        }
    }

    if any_failures {
        anyhow::bail!("Some feature dependencies are not met");
    }

    Ok(())
}
