use std::fs;
use std::io;
//use std::path::PathBuf;

fn main() {
    let mut project_path = String::new();
    println!("Введите расположение проекта:");
    io::stdin()
        .read_line(&mut project_path)
        .expect("Failed to read line");
    project_path.pop();
    project_path.pop();

    let altium_gerber_path = format!("{}/Outputs/Gerber", project_path);
    let altium_drill_path = format!("{}/Outputs/NC Drill", project_path);
    let docs_path = format!("{}/RustTest", project_path);
    let name_project = find_name(&project_path);

    copy_gerbers(&altium_gerber_path, &docs_path, &name_project);
    copy_drills(&altium_drill_path, &docs_path, &name_project);
}

fn copy_gerbers(from: &str, to: &str, name: &str) {
    fs::copy(
        gerber_path(from, name, ".GBL"),
        gerber_path(to, name, ".GBL"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GBO"),
        gerber_path(to, name, ".GBO"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GBP"),
        gerber_path(to, name, ".GBP"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GBS"),
        gerber_path(to, name, ".GBS"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GKO"),
        gerber_path(to, name, ".GKO"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GTL"),
        gerber_path(to, name, ".GTL"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GTO"),
        gerber_path(to, name, ".GTO"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GTP"),
        gerber_path(to, name, ".GTP"),
    )
    .unwrap();

    fs::copy(
        gerber_path(from, name, ".GTS"),
        gerber_path(to, name, ".GTS"),
    )
    .unwrap();
}

fn copy_drills(from: &str, to: &str, name: &str) {
    fs::copy(
        gerber_path(from, name, ".TXT"),
        gerber_path(to, name, ".TXT"),
    )
    .unwrap();
}

fn gerber_path(folder: &str, project_name: &str, extension: &str) -> String {
    format!("{}/{}{}", folder, project_name, extension)
}

fn find_name(project_path: &str) -> String {

    let mut name = "None".to_string();

    for file in fs::read_dir(project_path).unwrap() {
        let file_name = file.unwrap().file_name().into_string().unwrap();

        if file_name.contains(".PrjPcb") {
            name = file_name.split('.').next().unwrap().to_string();
        }
    }
    return name.to_string();
}
