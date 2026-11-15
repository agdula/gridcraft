//! Application updates are managed centrally by the owner.
//!
//! Keep the host entry points as no-ops for compatibility, but never construct an
//! updater, contact a release service, display an update dialog, or self-replace
//! the executable. This preserves reviewed local customizations across updates.

use crate::SheetApp;

pub(crate) fn start(_app: &mut SheetApp, _manual: bool) {}
