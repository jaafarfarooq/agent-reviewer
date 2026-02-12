/// Agent module: collection of review agents and helpers.
///
/// Responsibilities:
/// - Expose agent implementations (style, bug, performance, meta)
/// - Export prompts and the `ReviewAgent` trait for consumers
pub mod style;
pub mod bug;
pub mod performance;
pub mod r#trait;
pub mod prompts;
pub mod meta_agent;
