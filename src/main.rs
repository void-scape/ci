use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args();
    let binary = args.next().unwrap();
    let workflow_name = match args.next() {
        Some(name) => name,
        None => {
            println!(
                "Usage: {binary} workflow-name [-d<DEPENDENCIES>] {}",
                ci::available_jobs()
                    .iter()
                    .map(|job| format!("[{job}]"))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            return ExitCode::FAILURE;
        }
    };

    let mut peekable = args.peekable();
    let (jobs, deps) = if peekable.peek().is_some_and(|deps| deps.starts_with("-d")) {
        let deps = peekable
            .next()
            .unwrap()
            .strip_prefix("-d")
            .unwrap()
            .to_string();
        (peekable, Some(deps))
    } else {
        (peekable, None)
    };

    let workflow =
        match ci::generate_workflow(&workflow_name, jobs, deps.as_ref().map(|d| d.as_ref())) {
            Ok(workflow) => workflow,
            Err(err) => {
                println!("{binary}: {err}");
                return ExitCode::FAILURE;
            }
        };

    let workflow_parent = std::path::PathBuf::from(".github/workflows/");
    std::fs::create_dir_all(&workflow_parent)
        .expect("failed to create '.github/workflows' directory");
    let path = workflow_parent.join(format!("{workflow_name}.yaml"));
    std::fs::write(&path, &workflow).expect("failed to write workflow");
    println!("{binary}: wrote {} bytes to {path:?}", workflow.len());

    ExitCode::SUCCESS
}
