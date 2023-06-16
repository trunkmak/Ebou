use super::TimelineProvider;
use crate::{
    environment::{types::TimelineDirection, Environment},
    view_model::{StatusId, StatusViewModel},
};
use futures_util::Future;
use megalodon::entities::Status;
use std::pin::Pin;

/// A provider that just loads the users bookmarks
pub struct PublicTimelineProvider {
    environment: Environment,
}

impl PublicTimelineProvider {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }
}

impl std::fmt::Debug for PublicTimelineProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PublicTimelineProvider").finish()
    }
}

impl TimelineProvider for PublicTimelineProvider {
    type Id = StatusId;
    type Element = Status;
    type ViewModel = StatusViewModel;
    fn should_auto_reload(&self) -> bool {
        true
    }

    fn identifier(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn forced_direction(&self) -> Option<TimelineDirection> {
        Some(TimelineDirection::NewestTop)
    }

    fn reset(&self) {
        self.environment.storage.with_mutation(|mut storage| {
            storage.public_timeline.clear();
        })
    }

    fn scroll_to_item(&self, _updates: &[Status]) -> Option<StatusId> {
        None
    }

    fn request_data(
        &self,
        after: Option<StatusId>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Status>, String>> + Send>> {
        let after = after.map(|e| e.0);
        let model = self.environment.model.clone();
        Box::pin(async move { model.public_timeline(after).await })
    }

    fn process_new_data(
        &self,
        updates: &[Status],
        _direction: TimelineDirection,
        is_reload: bool,
    ) -> bool {
        let can_load_more = !updates.is_empty();
        self.environment
            .storage
            .with_mutation(|mut storage| storage.merge_publictimeline(updates, is_reload));
        can_load_more
    }

    fn data(&self, _direction: TimelineDirection) -> Vec<StatusViewModel> {
        self.environment
            .storage
            .with(|storage| storage.public_timeline.clone())
    }
}
