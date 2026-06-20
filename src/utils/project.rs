use crate::completion::completion::Completion;
use crate::completion::completion_doc::CompletionDocData;
use crate::utils::join_relate_to_cwd;
use crate::utils::project_config::ProjectsConfig;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Project {
    pub start_path: String,
    pub completion: Completion,
}
#[derive(Debug)]
pub struct Projects {
    pub projects: Vec<Project>,
}
impl Projects {
    pub fn get_project(&self, path: &str) -> Option<Project> {
        tracing::info!("searching project for path {}", path,);
        for project in &self.projects {
            if path.starts_with(&project.start_path) {
                return Some(project.clone());
            }
        }
        None
    }
}
impl From<ProjectsConfig> for Projects {
    fn from(value: ProjectsConfig) -> Self {
        let loaded_projects = value;

        let available_locales = vec!["en".to_string(), "fr".to_string()];

        let mut projects = Vec::new();
        for project in loaded_projects.project {
            let mut doc = HashMap::new();
            doc.insert(
                "en".to_string(),
                CompletionDocData::new("This is a test", "./locales/en.toml"),
            );
            let mut completions = Completion::new();
            completions.set_available_locales(&available_locales);
            completions.add_completion(crate::completion::completion_data::CompletionData::new(
                "hello.world",
                doc,
            ));
            projects.push(Project {
                start_path: join_relate_to_cwd(&*project.root)
                    .to_string_lossy()
                    .clone()
                    .to_string(),
                completion: completions,
            });
        }
        Projects { projects }
    }
}
