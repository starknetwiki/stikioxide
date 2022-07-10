use std::collections::HashMap;

pub fn format_stiki_markdown(body: String, refs: HashMap<String, String>) -> String {
    let mut response: String = body.clone();
    for (artifact_name, artifact_value) in refs.iter() {
        response = response.replace(artifact_name, artifact_value);
    }
    response
}
