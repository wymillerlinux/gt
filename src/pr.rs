use serde_derive::{Deserialize, Serialize};

pub struct PullRequest;

#[derive(Serialize, Deserialize)]
pub struct MultiplePullRequests {
    pub data: Vec<PR>,
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PR {
    pub assignee: Assignee,
    pub assignees: Vec<Assignee2>,
    pub base: Base,
    pub body: String,
    #[serde(rename = "closed_at")]
    pub closed_at: String,
    pub comments: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "due_date")]
    pub due_date: String,
    pub head: Head,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    #[serde(rename = "is_locked")]
    pub is_locked: bool,
    pub labels: Vec<Label>,
    #[serde(rename = "merge_base")]
    pub merge_base: String,
    #[serde(rename = "merge_commit_sha")]
    pub merge_commit_sha: String,
    pub mergeable: bool,
    pub merged: bool,
    #[serde(rename = "merged_at")]
    pub merged_at: String,
    #[serde(rename = "merged_by")]
    pub merged_by: MergedBy,
    pub milestone: Milestone,
    pub number: i64,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
    pub state: String,
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignee {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignee2 {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub repo: Repo,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    pub sha: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    #[serde(rename = "allow_merge_commits")]
    pub allow_merge_commits: bool,
    #[serde(rename = "allow_rebase")]
    pub allow_rebase: bool,
    #[serde(rename = "allow_rebase_explicit")]
    pub allow_rebase_explicit: bool,
    #[serde(rename = "allow_squash_merge")]
    pub allow_squash_merge: bool,
    pub archived: bool,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "default_merge_style")]
    pub default_merge_style: String,
    pub description: String,
    pub empty: bool,
    #[serde(rename = "external_tracker")]
    pub external_tracker: ExternalTracker,
    #[serde(rename = "external_wiki")]
    pub external_wiki: ExternalWiki,
    pub fork: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(rename = "has_pull_requests")]
    pub has_pull_requests: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    #[serde(rename = "ignore_whitespace_conflicts")]
    pub ignore_whitespace_conflicts: bool,
    pub internal: bool,
    #[serde(rename = "internal_tracker")]
    pub internal_tracker: InternalTracker,
    pub mirror: bool,
    #[serde(rename = "mirror_interval")]
    pub mirror_interval: String,
    pub name: String,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    #[serde(rename = "open_pr_counter")]
    pub open_pr_counter: i64,
    #[serde(rename = "original_url")]
    pub original_url: String,
    pub owner: Owner,
    pub permissions: Permissions,
    pub private: bool,
    #[serde(rename = "release_counter")]
    pub release_counter: i64,
    pub size: i64,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "stars_count")]
    pub stars_count: i64,
    pub template: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub website: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalTracker {
    #[serde(rename = "external_tracker_format")]
    pub external_tracker_format: String,
    #[serde(rename = "external_tracker_style")]
    pub external_tracker_style: String,
    #[serde(rename = "external_tracker_url")]
    pub external_tracker_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalWiki {
    #[serde(rename = "external_wiki_url")]
    pub external_wiki_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTracker {
    #[serde(rename = "allow_only_contributors_to_track_time")]
    pub allow_only_contributors_to_track_time: bool,
    #[serde(rename = "enable_issue_dependencies")]
    pub enable_issue_dependencies: bool,
    #[serde(rename = "enable_time_tracker")]
    pub enable_time_tracker: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub repo: Repo2,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    pub sha: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo2 {
    #[serde(rename = "allow_merge_commits")]
    pub allow_merge_commits: bool,
    #[serde(rename = "allow_rebase")]
    pub allow_rebase: bool,
    #[serde(rename = "allow_rebase_explicit")]
    pub allow_rebase_explicit: bool,
    #[serde(rename = "allow_squash_merge")]
    pub allow_squash_merge: bool,
    pub archived: bool,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "default_merge_style")]
    pub default_merge_style: String,
    pub description: String,
    pub empty: bool,
    #[serde(rename = "external_tracker")]
    pub external_tracker: ExternalTracker2,
    #[serde(rename = "external_wiki")]
    pub external_wiki: ExternalWiki2,
    pub fork: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(rename = "has_pull_requests")]
    pub has_pull_requests: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    #[serde(rename = "ignore_whitespace_conflicts")]
    pub ignore_whitespace_conflicts: bool,
    pub internal: bool,
    #[serde(rename = "internal_tracker")]
    pub internal_tracker: InternalTracker2,
    pub mirror: bool,
    #[serde(rename = "mirror_interval")]
    pub mirror_interval: String,
    pub name: String,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    #[serde(rename = "open_pr_counter")]
    pub open_pr_counter: i64,
    #[serde(rename = "original_url")]
    pub original_url: String,
    pub owner: Owner2,
    pub permissions: Permissions2,
    pub private: bool,
    #[serde(rename = "release_counter")]
    pub release_counter: i64,
    pub size: i64,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "stars_count")]
    pub stars_count: i64,
    pub template: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub website: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalTracker2 {
    #[serde(rename = "external_tracker_format")]
    pub external_tracker_format: String,
    #[serde(rename = "external_tracker_style")]
    pub external_tracker_style: String,
    #[serde(rename = "external_tracker_url")]
    pub external_tracker_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalWiki2 {
    #[serde(rename = "external_wiki_url")]
    pub external_wiki_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTracker2 {
    #[serde(rename = "allow_only_contributors_to_track_time")]
    pub allow_only_contributors_to_track_time: bool,
    #[serde(rename = "enable_issue_dependencies")]
    pub enable_issue_dependencies: bool,
    #[serde(rename = "enable_time_tracker")]
    pub enable_time_tracker: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner2 {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions2 {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub color: String,
    pub description: String,
    pub id: i64,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedBy {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    #[serde(rename = "closed_at")]
    pub closed_at: String,
    #[serde(rename = "closed_issues")]
    pub closed_issues: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub description: String,
    #[serde(rename = "due_on")]
    pub due_on: String,
    pub id: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub state: String,
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: i64,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    pub language: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    pub login: String,
    pub restricted: bool,
}