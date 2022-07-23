const TEMPLATE_REPLACE_VARIABLE: &'static str = r#"\{\{TEMPLATE\}\}"#;
const FILENMAE_REPLACE_VARIABLE: &'static str = r#"\{\{STEM\}\}"#;
const TRAVERSE_COUNT_REPLACE_VARIABLE: &'static str = r#"\{\{TRAVERSE_COUNT\}\}"#;

pub fn filter_target_files(
    target: &PathBuf,
    target_extension: &str,
) -> Vec<Result<DirEntry, Error>> {
    let dir = fs::read_dir(target).expect("Please make sure the target directory exists");
    let dir: Vec<Result<DirEntry, Error>> = dir
        .filter(|dir_entry| {
            let dir_entry = dir_entry.as_ref().unwrap();
            let dir_entry_path = dir_entry.path();
            let extension = match dir_entry_path.extension() {
                Some(extension) => extension.to_str().unwrap(),
                None => "",
            };

            extension == target_extension
        })
        .collect();

    dir
}

pub fn translate_target_files(
    template: &PathBuf,
    dist: &PathBuf,
    target: &PathBuf,
    target_extension: &str,
    dist_extension: &str,
) {
    let template_content =
        fs::read_to_string(template).expect("Please make sure the template file exists");
    let dir = filter_target_files(target, target_extension);
    let regex = Regex::new(TEMPLATE_REPLACE_VARIABLE).unwrap();

    for dir_entry in dir {
        let dir_entry = dir_entry.unwrap();
        let dir_entry_path = dir_entry.path();
        let dir_entry_filename = dir_entry_path.file_stem().unwrap();
        let dir_content = fs::read_to_string(&dir_entry_path).unwrap();
        let replaced_content = regex.replace_all(&template_content, &dir_content);
        let dist_path = dist.join(format!(
            "{}.{}",
            dir_entry_filename.to_str().unwrap(),
            dist_extension
        ));

        fs::write(&dist_path, replaced_content.as_bytes()).unwrap();
    }
}

fn reduce_target_files(
    template: &PathBuf,
    iteration_template: &PathBuf,
    dist_filename: &PathBuf,
    target: &PathBuf,
    target_extension: &str,
) {
    let template_regex = Regex::new(TEMPLATE_REPLACE_VARIABLE).unwrap();
    let stem_regex = Regex::new(FILENMAE_REPLACE_VARIABLE).unwrap();
    let traverse_count_regex = Regex::new(TRAVERSE_COUNT_REPLACE_VARIABLE).unwrap();
    let template_content =
        fs::read_to_string(template).expect("Please make sure the template file exists");
    let iteration_template_content = fs::read_to_string(iteration_template)
        .expect("Please make sure the iteration template file exists");
    let dir = filter_target_files(target, target_extension);
    let mut wait_for_insert_template_content = "".to_string();
    let traverse_count = dir.len();

    for dir_entry in dir {
        let dir_entry = dir_entry.unwrap();
        let dir_entry_path = dir_entry.path();
        let dir_entry_filename = dir_entry_path.file_stem().unwrap().to_str().unwrap();
        let dist_filename = dist_filename.file_stem().unwrap().to_str().unwrap();

        if dist_filename == dir_entry_filename {
            continue;
        }

        let dir_content = fs::read_to_string(&dir_entry_path).unwrap();
        let wait_for_insert_content = stem_regex
            .replace_all(&iteration_template_content, dir_entry_filename)
            .into_owned();
        let wait_for_insert_content = template_regex
            .replace_all(wait_for_insert_content.as_str(), dir_content)
            .into_owned();

        wait_for_insert_template_content += wait_for_insert_content.as_str();
    }

    let dist_content = template_regex
        .replace_all(&template_content, &wait_for_insert_template_content)
        .into_owned();
    let dist_content = traverse_count_regex
        .replace_all(dist_content.as_str(), traverse_count.to_string())
        .into_owned();

    fs::write(&dist_filename, dist_content.as_str()).unwrap();
}

fn main() {
    let args = CLI::parse();

    match args.action {
        Action::Translate {
            template,
            dist,
            target,
            extension,
            dist_extension,
        } => translate_target_files(&template, &dist, &target, &extension, &dist_extension),
        Action::Reduce {
            template,
            iteration_template,
            dist_filename,
            target,
            extension,
        } => reduce_target_files(
            &template,
            &iteration_template,
            &dist_filename,
            &target,
            &extension,
        ),
    }
}
