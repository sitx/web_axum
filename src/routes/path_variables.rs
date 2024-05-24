use axum::extract::Path;

pub async fn path_variables(Path(id):Path<String>) -> String {
    id.to_owned()

}

pub async fn hardcoded_path() -> String {
    "это захардкоджено: id=15".to_owned()
}