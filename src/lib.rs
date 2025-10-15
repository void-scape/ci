pub fn available_jobs() -> &'static [&'static str] {
    &["check", "clippy", "fmt", "test", "typos", "doc"]
}

pub fn generate_workflow(
    workflow_name: &str,
    jobs: impl Iterator<Item: AsRef<str>>,
    dependencies: Option<&str>,
) -> Result<String, String> {
    let deps = dependencies
        .map(install_dependencies)
        .unwrap_or_else(|| String::from("# no dependencies"));
    let mut workflow = include_str!("header.yaml").replace("<name>", workflow_name);
    let mut jobs_prefix = false;
    for job in jobs {
        if !jobs_prefix {
            jobs_prefix = true;
            workflow.push_str("jobs:\n");
        }
        match job.as_ref() {
            "check" => workflow.push_str(&include_str!("check.yaml").replace("<dep>", &deps)),
            "clippy" => workflow.push_str(&include_str!("clippy.yaml").replace("<dep>", &deps)),
            "fmt" => workflow.push_str(include_str!("fmt.yaml")),
            "test" => workflow.push_str(&include_str!("test.yaml").replace("<dep>", &deps)),
            "typos" => workflow.push_str(include_str!("typos.yaml")),
            "doc" => workflow.push_str(include_str!("doc.yaml")),
            job => {
                return Err(format!("unrecognized job {job}"));
            }
        }
    }
    if !jobs_prefix {
        return Err("expected at least one job".to_string());
    }
    Ok(workflow)
}

fn install_dependencies(dependencies: &str) -> String {
    format!(
        "- name: Install Dependencies\n        run: \
      sudo apt-get update; sudo apt-get install \
      --no-install-recommends {dependencies}"
    )
}
