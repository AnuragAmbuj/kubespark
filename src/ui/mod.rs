pub mod dashboard;
pub mod detail_view;
pub mod glass;
pub mod logs;
pub mod resource_list;
pub mod sidebar;
pub mod status_bar;

pub use dashboard::DashboardView;
pub use detail_view::DetailView;
pub use glass::GlassStyle;
pub use logs::LogView;
pub use resource_list::ResourceListView;
pub use sidebar::Sidebar;
// pub use status_bar::StatusBar;

use crate::kubernetes::ResourceKind;

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveView {
    Dashboard,
    Resources(ResourceKind),
    Logs(LogView), // Component struct, not View handle
}
