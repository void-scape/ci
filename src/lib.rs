pub fn available_jobs() -> &'static [&'static str] {
    &["check", "clippy", "fmt", "test", "typos"]
}

pub fn generate_workflow(
    workflow_name: &str,
    jobs: impl Iterator<Item: AsRef<str>>,
) -> Result<String, String> {
    let mut workflow = include_str!("header.yaml").replace("<name>", workflow_name);
    let mut jobs_prefix = false;
    for job in jobs {
        if !jobs_prefix {
            jobs_prefix = true;
            workflow.push_str("jobs:\n");
        }
        match job.as_ref() {
            "check" => workflow.push_str(include_str!("check.yaml")),
            "clippy" => workflow.push_str(include_str!("clippy.yaml")),
            "fmt" => workflow.push_str(include_str!("fmt.yaml")),
            "test" => workflow.push_str(include_str!("test.yaml")),
            "typos" => workflow.push_str(include_str!("typos.yaml")),
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
