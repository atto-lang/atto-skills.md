//! Dependency-free, fail-closed Agent Skills documentation validator.
//! Build: rustc --edition=2021 scripts/validate_skills.rs -o /tmp/atto-validate-skills
use std::{collections::BTreeMap, env, fs, path::{Path, PathBuf}, process};

const MAX_DOC_BYTES: u64 = 1024 * 1024;

fn read_text(path: &Path) -> Result<String, String> {
    let info = fs::metadata(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    if !info.is_file() || info.len() > MAX_DOC_BYTES {
        return Err(format!("{}: not a regular UTF-8 file <=1 MiB", path.display()));
    }
    fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))
}

fn valid_name(name: &str) -> bool {
    name.len() <= 64 && !name.is_empty() &&
    name.split('-').all(|piece| !piece.is_empty() &&
        piece.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
}

fn validate_frontmatter(path: &Path, body: &str, errors: &mut Vec<String>) {
    let mut lines = body.lines();
    if lines.next().map(str::trim_end) != Some("---") {
        errors.push(format!("{}: missing YAML frontmatter", path.display()));
        return;
    }
    let mut fields = BTreeMap::new();
    let mut closed = false;
    for line in lines.by_ref() {
        if line.trim_end() == "---" { closed = true; break; }
        let Some((key, value)) = line.split_once(':') else {
            errors.push(format!("{}: invalid frontmatter entry", path.display()));
            continue;
        };
        if !key.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') ||
           key.is_empty() || value.trim().is_empty() {
            errors.push(format!("{}: invalid frontmatter key/value", path.display()));
            continue;
        }
        if fields.insert(key.to_string(), value.trim().to_string()).is_some() {
            errors.push(format!("{}: duplicate YAML frontmatter key {}", path.display(), key));
        }
    }
    if !closed {
        errors.push(format!("{}: missing closing YAML frontmatter", path.display()));
        return;
    }
    let name = fields.get("name").map(String::as_str).unwrap_or("");
    let expected = path.parent().and_then(Path::file_name)
        .and_then(|s| s.to_str()).unwrap_or("");
    if !valid_name(name) || name != expected {
        errors.push(format!("{}: invalid/mismatched skill name", path.display()));
    }
    let description = fields.get("description").map(String::as_str).unwrap_or("");
    let d = description.to_ascii_lowercase();
    if description.is_empty() || description.len() > 1024 ||
       !(d.contains("use when") || d.contains("use for")) {
        errors.push(format!("{}: invalid description/activation context", path.display()));
    }
    let instruction_start = body.lines().position(|line| line.trim_end() == "---");
    let actual_instructions = instruction_start
        .and_then(|_| body.splitn(3, "---").nth(2))
        .unwrap_or("").trim();
    if actual_instructions.len() < 100 || !actual_instructions.starts_with("# ") {
        errors.push(format!("{}: missing substantive skill instructions", path.display()));
    }
    if actual_instructions.matches("~~~").count() % 2 != 0 {
        errors.push(format!("{}: unclosed fenced code block", path.display()));
    }
}

fn markdown_link_targets(body: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut tail = body;
    while let Some(i) = tail.find("](") {
        let before = &tail[..i];
        let after = &tail[i+2..];
        if before.rfind('[').is_some() {
            if let Some(end) = after.find(')') {
                result.push(&after[..end]);
                tail = &after[end+1..];
                continue;
            }
        }
        tail = after;
    }
    result
}

fn check_links(root: &Path, doc: &Path, body: &str, errors: &mut Vec<String>) {
    for target in markdown_link_targets(body) {
        let rel = target.split('#').next().unwrap_or("").split('?').next().unwrap_or("");
        if rel.is_empty() || rel.contains("://") || rel.starts_with("mailto:") ||
            rel.starts_with('/') { continue; }
        let candidate = doc.parent().unwrap_or(root).join(rel);
        match fs::canonicalize(&candidate) {
            Ok(found) if found.starts_with(root) && found.is_file() => {},
            _ => errors.push(format!("{}: invalid local link {}", doc.display(), target))
        }
    }
}

fn validate(root: &Path) -> (usize, Vec<String>) {
    let mut errors = Vec::new();
    let canonical_root = match fs::canonicalize(root) {
        Ok(p) => p,
        Err(e) => return (0, vec![format!("Invalid repository root: {e}")])
    };
    let skills_dir = canonical_root.join("skills");
    let mut skills: Vec<PathBuf> = match fs::read_dir(&skills_dir) {
        Ok(entries) => entries.filter_map(Result::ok).map(|e| e.path().join("SKILL.md"))
            .filter(|p| p.is_file()).collect(),
        Err(e) => return (0, vec![format!("Cannot read skills directory: {e}")])
    };
    skills.sort();
    if skills.is_empty() { errors.push("No skills/*/SKILL.md files found".into()); }
    for path in &skills {
        match read_text(path) {
            Ok(body) => {
                validate_frontmatter(path, &body, &mut errors);
                check_links(&canonical_root, path, &body, &mut errors);
            },
            Err(e) => errors.push(e)
        }
    }
    let mut catalog = String::new();
    for rel in ["README.md", "AGENTS.md", "docs/FEATURE_STATUS.md"] {
        let path = canonical_root.join(rel);
        match read_text(&path) {
            Ok(body) => {
                check_links(&canonical_root, &path, &body, &mut errors);
                if rel == "README.md" { catalog = body; }
            },
            Err(_) => errors.push(format!("Missing or unreadable {}", rel))
        }
    }
    for path in &skills {
        let Some(name) = path.parent().and_then(Path::file_name).and_then(|x| x.to_str()) else {
            continue;
        };
        if !catalog.contains(&format!("skills/{name}/SKILL.md")) {
            errors.push(format!("README.md: not indexed: {}", name));
        }
    }
    (skills.len(), errors)
}

fn main() {
    let root = env::current_dir().expect("Need a readable working directory");
    let (count, errors) = validate(&root);
    if errors.is_empty() {
        println!("Validated {} Agent Skills and local documentation links.", count);
    } else {
        eprintln!("Skill validation failed:");
        for e in errors { eprintln!(" - {}", e); }
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempRoot(PathBuf);
    impl TempRoot {
        fn new() -> Self {
            let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let path = env::temp_dir().join(format!("atto-skills-{}-{nonce}", process::id()));
            fs::create_dir_all(path.join("skills/example-skill")).unwrap();
            fs::create_dir_all(path.join("docs")).unwrap();
            fs::write(path.join("README.md"), "[Skill](skills/example-skill/SKILL.md)").unwrap();
            fs::write(path.join("AGENTS.md"), "# Agents\n").unwrap();
            fs::write(path.join("docs/FEATURE_STATUS.md"), "# Status\n").unwrap();
            fs::write(path.join("skills/example-skill/SKILL.md"), Self::skill()).unwrap();
            Self(path)
        }
        fn skill() -> String {
            format!("---\nname: example-skill\ndescription: Use for validating AttoLang contracts\n---\n\n# Example\n\n{}\n", "Checked and bounded source. ".repeat(8))
        }
    }
    impl Drop for TempRoot {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }

    #[test] fn names_are_canonical_and_bounded() {
        assert!(valid_name("atto-borrow-checker"));
        for bad in ["", "-a", "a-", "a--b", "UPPER", "a_b", "a.b"] {
            assert!(!valid_name(bad), "{bad}");
        }
    }
    #[test] fn valid_fixture_passes() {
        let t = TempRoot::new();
        assert_eq!(validate(&t.0).1, Vec::<String>::new());
    }
    #[test] fn duplicate_and_missing_activation_fail() {
        let t = TempRoot::new();
        let p = t.0.join("skills/example-skill/SKILL.md");
        fs::write(&p, TempRoot::skill().replace("description: Use for", "description: inactive\nname: duplicate\nextra: yes\nnote: Use for")).unwrap();
        assert!(validate(&t.0).1.iter().any(|x| x.contains("duplicate YAML")));
    }
    #[test] fn external_traversal_and_missing_links_fail_closed() {
        let t = TempRoot::new();
        let p = t.0.join("README.md");
        fs::write(p, "[Skill](skills/example-skill/SKILL.md) [Bad](../secret.txt) [Missing](missing.md)").unwrap();
        let errors = validate(&t.0).1;
        assert!(errors.iter().any(|x| x.contains("../secret.txt")));
        assert!(errors.iter().any(|x| x.contains("missing.md")));
    }
    #[test] fn catalog_omission_is_rejected() {
        let t = TempRoot::new();
        fs::write(t.0.join("README.md"), "# Empty\n").unwrap();
        assert!(validate(&t.0).1.iter().any(|x| x.contains("not indexed")));
    }
    #[test] fn wrong_name_and_unclosed_fence_reject() {
        let t = TempRoot::new();
        let p = t.0.join("skills/example-skill/SKILL.md");
        fs::write(&p, TempRoot::skill().replace("name: example-skill", "name: other-name")+"~~~atto\n").unwrap();
        let errors=validate(&t.0).1;
        assert!(errors.iter().any(|x| x.contains("mismatched")));
        assert!(errors.iter().any(|x| x.contains("unclosed fenced")));
    }
}
