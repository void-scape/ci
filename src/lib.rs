pub fn generate_workflow(
    workflow_name: &str,
    jobs: impl Iterator<Item: AsRef<str>>,
) -> Result<String, String> {
    let mut workflow = include_str!("header.yaml").replace("<name>", workflow_name);
    let mut jobs_prefix = false;
    for job in jobs {
        if !jobs_prefix {
            jobs_prefix = true;
            workflow.push_str("\njobs:\n");
        }
        match job.as_ref() {
            "test" => workflow.push_str(include_str!("test.yaml")),
            "lint" => workflow.push_str(include_str!("lint.yaml")),
            "check" => workflow.push_str(include_str!("check.yaml")),
            job => {
                return Err(format!("unrecognized job {job}"));
            }
        }
    }
    Ok(workflow)
}
