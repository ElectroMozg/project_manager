use std::fs;
use std::io;
//use std::path::PathBuf;

fn main() {

    // let mut project_path = String::new();
    // println!("Введите расположение проекта:");
    // io::stdin()
    //     .read_line(&mut project_path)
    //     .expect("Failed to read line");
    // project_path.pop();
    // project_path.pop();

    // let altium_gerber_path = format!("{}/Outputs/Gerber", project_path);
    // let altium_drill_path = format!("{}/Outputs/NC Drill", project_path);
    // let docs_path = format!("{}/RustTest", project_path);
    //let name_project = find_name(&project_path);

    //copy_gerbers(&altium_gerber_path, &docs_path, &name_project);
    //copy_drills(&altium_drill_path, &docs_path, &name_project);
}

pub struct Gerber {
    path_from: String,
    path_to: String,
    project_name: String,
}

impl Gerber {
    fn new(path_from: String, path_to: String) -> Gerber {
        let mut obj = Gerber {
            path_from,
            path_to,
            project_name: String::new(),
        };

        obj.project_name = obj.find_name(&obj.path_from);
        obj
    }

    pub fn copy_gerbers(&self) {
        self.copy_with_ext(".GBL");
        self.copy_with_ext(".GBO");
        self.copy_with_ext(".GBP");
        self.copy_with_ext(".GBS");
        self.copy_with_ext(".GKO");
        self.copy_with_ext(".GTL");
        self.copy_with_ext(".GTO");
        self.copy_with_ext(".GTP");
        self.copy_with_ext(".GTS");
    }

    pub fn copy_drills(&mut self) {
        self.copy_with_ext(".TXT");
    }

    fn copy_with_ext(&self, extension: &str) {
        fs::copy(
            gerber_path(&self.path_from, &self.project_name, extension),
            gerber_path(&self.path_to, &self.project_name, extension),
        )
        .unwrap();
    }

    fn find_name(&self, project_path: &str) -> String {
        let mut name = "None".to_string();

        for file in fs::read_dir(project_path).unwrap() {
            let file_name = file.unwrap().file_name().into_string().unwrap();

            if file_name.contains(".PrjPcb") {
                name = file_name.split('.').next().unwrap().to_string();
            }
        }
        return name.to_string();
    }
}

fn gerber_path(folder: &str, project_name: &str, extension: &str) -> String {
    format!("{}/{}{}", folder, project_name, extension)
}


