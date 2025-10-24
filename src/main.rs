use inquire::Select;

fn main() {
    let projects = vec!["Project A", "Project B", "Project C"];

    loop {
        let project = Select::new("Select a project:", projects.clone()).prompt();

        let Ok(project) = project else { break };

        let subprojects = match project {
            "Project A" => vec!["Sub A1", "Sub A2", "<- Back"],
            "Project B" => vec!["Sub B1", "Sub B2", "<- Back"],
            "Project C" => vec!["Sub C1", "<- Back"],
            _ => vec![],
        };

        let subproject = Select::new("Select a subproject:", subprojects)
            .prompt()
            .unwrap();

        if subproject == "<- Back" {
            continue; // go back to project selection
        }

        println!("You selected {project} -> {subproject}");
        break;
    }
}
