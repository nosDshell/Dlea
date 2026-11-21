//! user-dirs.dirs read the way the rail lists it: roles in first-seen order, the last valid path per role.
use super::leaf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDir {
    pub path: String,
    pub label: String,
    pub label_key: &'static str,
    rank: usize,
}

// Only a role's standard folder name is product copy; renamed folders retain their own names.
fn label_key(role: &str, label: &str) -> &'static str {
    match (role, label) {
        ("DESKTOP", "Desktop") => "places.row.desktop",
        ("DOCUMENTS", "Documents") => "places.row.documents",
        ("DOWNLOAD", "Downloads") => "places.row.downloads",
        ("MUSIC", "Music") => "places.row.music",
        ("PICTURES", "Pictures") => "places.row.pictures",
        ("VIDEOS", "Videos") => "places.row.videos",
        _ => "",
    }
}

// Sample input: XDG_DOWNLOAD_DIR="$HOME/Photo\"Inbox" # local inbox gives ("DOWNLOAD", Some(".../Photo\"Inbox")).
pub fn parse_line(line: &str, home: &str) -> Option<(String, Option<String>)> {
    let rest = line.trim_start_matches([' ', '\t']).strip_prefix("XDG_")?;
    let cut = rest.find("_DIR")?;
    let role = &rest[..cut];
    if role.is_empty() || !role.bytes().all(|b| b.is_ascii_uppercase()) {
        return None;
    }
    let value = rest[cut + 4..]
        .trim_start_matches([' ', '\t'])
        .strip_prefix('=')?;
    Some((role.to_owned(), quoted_path(value, home)))
}

fn quoted_path(value: &str, home: &str) -> Option<String> {
    let body = value.trim_start_matches([' ', '\t']).strip_prefix('"')?;
    let (from_home, body) = match body.strip_prefix("$HOME") {
        Some(after) if after.starts_with('/') || after.starts_with('"') => (true, after),
        _ => (false, body),
    };
    let mut raw = String::new();
    let mut chars = body.chars();
    loop {
        match chars.next()? {
            '\\' => raw.push(chars.next()?),
            '"' => break,
            c => raw.push(c),
        }
    }
    if !from_home && !body.starts_with('/') {
        return None;
    }
    let path = format!("{}{}", if from_home { home } else { "" }, raw);
    let path = path.trim_end_matches('/');
    path.starts_with('/').then(|| path.to_owned())
}

// Sample input: XDG_DOWNLOAD_DIR="$HOME/Old" then XDG_DOWNLOAD_DIR="$HOME/Inbox"; the last valid path wins.
pub fn read_roles(body: &str, home: &str) -> Vec<(String, String)> {
    let mut roles: Vec<String> = Vec::new();
    let mut paths: Vec<(String, String)> = Vec::new();
    for (role, path) in body.split('\n').filter_map(|line| parse_line(line, home)) {
        if !roles.contains(&role) {
            roles.push(role.clone());
        }
        if let Some(path) = path {
            paths.retain(|(own, _)| *own != role);
            paths.push((role, path));
        }
    }
    roles
        .into_iter()
        .filter_map(|role| paths.iter().find(|(own, _)| *own == role).cloned())
        .collect()
}

// gxde-file-manager's Common group lists exactly these roles, in this order; other roles are never places.
const GXDE_ORDER: [&str; 6] = [
    "DESKTOP",
    "VIDEOS",
    "MUSIC",
    "PICTURES",
    "DOCUMENTS",
    "DOWNLOAD",
];

fn gxde_rank(role: &str) -> usize {
    GXDE_ORDER
        .iter()
        .position(|known| *known == role)
        .unwrap_or(GXDE_ORDER.len())
}

/// The six standard folders when `user-dirs.dirs` is missing and the directory exists.
pub fn standard_dirs_body(home: &str) -> String {
    const ROLES: [(&str, &str); 6] = [
        ("DESKTOP", "Desktop"),
        ("VIDEOS", "Videos"),
        ("MUSIC", "Music"),
        ("PICTURES", "Pictures"),
        ("DOCUMENTS", "Documents"),
        ("DOWNLOAD", "Downloads"),
    ];
    let mut out = String::new();
    for (role, name) in ROLES {
        if std::path::Path::new(home).join(name).is_dir() {
            out.push_str(&format!("XDG_{role}_DIR=\"$HOME/{name}\"\n"));
        }
    }
    out
}

// Sample input: XDG_DOWNLOAD_DIR="$HOME/Downloads"; a disabled role that points at Home is no place of its own.
pub fn user_dirs(body: &str, home: &str) -> Vec<UserDir> {
    let mut dirs: Vec<UserDir> = read_roles(body, home)
        .into_iter()
        .filter(|(role, path)| path != home && gxde_rank(role) < GXDE_ORDER.len())
        .map(|(role, path)| UserDir {
            label_key: label_key(&role, leaf(&path)),
            label: leaf(&path).to_owned(),
            path,
            rank: gxde_rank(&role),
        })
        .collect();
    dirs.sort_by_key(|dir| dir.rank);
    dirs
}

#[cfg(test)]
#[path = "userdirs_language_tests.rs"]
mod language_tests;
