use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct AddonInfo {
    folder: String,
    title: Option<String>,
    version: Option<String>,
    interface: Option<String>,
    toc_path: PathBuf,
}

pub fn run(dir: &str) -> io::Result<()> {
    let path = Path::new(dir);

    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not a directory: {}", &dir),
        ));
    }

    let mut addons: Vec<AddonInfo> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if !ft.is_dir() {
            continue;
        }

        let folder_name = entry.file_name().to_string_lossy().to_string();
        let folder_path = entry.path();

        let toc_path = match find_toc_file(&folder_path, &folder_name)? {
            Some(p) => p,
            None => continue,
        };

        let toc_text = fs::read_to_string(&toc_path)?;
        let (title, version, interface) = parse_toc_metadata(&toc_text);

        addons.push(AddonInfo {
            folder: folder_name,
            title,
            version,
            interface,
            toc_path,
        });
    }

    addons.sort_by(|a, b| display_name(a).cmp(&display_name(b)));

    println!("Found {} addons in {}", addons.len(), &dir);
    for a in addons {
        let name = a.title.as_deref().unwrap_or(&a.folder);
        let ver = a.version.as_deref().unwrap_or("?");
        let iface = a.interface.as_deref().unwrap_or("?");
        println!(
            "{name}  (v{ver}, interface {iface})  [{folder}]",
            folder = a.folder
        );
    }

    Ok(())
}

fn display_name(a: &AddonInfo) -> String {
    a.title
        .as_ref()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| a.folder.to_lowercase())
}

fn find_toc_file(folder_path: &Path, folder_name: &str) -> io::Result<Option<PathBuf>> {
    let preferred = folder_path.join(format!("{folder_name}.toc"));
    if preferred.is_file() {
        return Ok(Some(preferred));
    }

    let mut first: Option<PathBuf> = None;
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if !ft.is_file() {
            continue;
        }

        let path = entry.path();
        if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("toc"))
        {
            first = Some(path);
            break;
        }
    }

    Ok(first)
}

fn parse_toc_metadata(toc_text: &str) -> (Option<String>, Option<String>, Option<String>) {
    let mut title: Option<String> = None;
    let mut version: Option<String> = None;
    let mut interface: Option<String> = None;

    for raw_line in toc_text.lines() {
        let line = raw_line.trim();

        if !line.starts_with("##") {
            continue;
        }

        if let Some(v) = parse_toc_field(line, "Title") {
            title = Some(v);
        } else if let Some(v) = parse_toc_field(line, "Version") {
            version = Some(v);
        } else if let Some(v) = parse_toc_field(line, "Interface") {
            interface = Some(v);
        }

        if title.is_some() && version.is_some() && interface.is_some() {
            break;
        }
    }

    (title, version, interface)
}

fn parse_toc_field(line: &str, key: &str) -> Option<String> {
    let prefix = format!("## {key}");
    if !line.starts_with(&prefix) {
        return None;
    }

    let rest = line[prefix.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim();
    if rest.is_empty() {
        None
    } else {
        Some(rest.to_string())
    }
}
