use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args();
    let binary = args.next().unwrap();
    let workflow_name = match args.next() {
        Some(name) => name,
        None => {
            println!("Usage: {binary} workflow-name [test] [lint] [check]");
            return ExitCode::FAILURE;
        }
    };

    let workflow = match ci::generate_workflow(&workflow_name, args) {
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
