/// Represents the status of a Git repository.
pub enum RepoStatus {
    Uncommitted,
    NoUpstream,
    NotPushed,
    Ok,
}
