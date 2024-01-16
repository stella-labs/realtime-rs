use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PresenceEvent {
    Track,
    Untrack,
    Join,
    Leave,
    Sync,
}

pub type RawPresenceState = HashMap<String, PresenceMetas>;

//{
//  abc123: {1: {foo: bar}, 2: {foo: baz} },
//  def456: {3: {foo: baz}, 4: {foo: bar} },
//}
//
// triple nested hashmap, fantastic. gonna need to write some helper functions for this one
pub type PresenceStateInner = HashMap<String, HashMap<String, HashMap<String, Value>>>;

/// HashMap<id, HashMap<phx_ref, HashMap<key, value>>>
/// { [id]: { [ref]: { [key]: value } } }
#[derive(Default, Clone, Debug)]
pub struct PresenceState(PresenceStateInner);

type PresenceIteratorItem = (String, HashMap<String, HashMap<String, Value>>);

impl FromIterator<PresenceIteratorItem> for PresenceState {
    fn from_iter<T: IntoIterator<Item = PresenceIteratorItem>>(iter: T) -> Self {
        let mut new_id_map = HashMap::new();

        for (id, id_map) in iter {
            new_id_map.insert(id, id_map);
        }

        PresenceState(new_id_map)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PresenceMeta {
    pub phx_ref: String,
    #[serde(flatten)]
    pub state_data: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PresenceMetas {
    pub metas: Vec<PresenceMeta>,
}

impl Into<PresenceState> for RawPresenceState {
    fn into(self) -> PresenceState {
        let mut transformed_state = PresenceState(HashMap::new());

        for (id, metas) in self {
            let mut transformed_inner = HashMap::new();

            for meta in metas.metas {
                transformed_inner.insert(meta.phx_ref, meta.state_data);
            }

            transformed_state.0.insert(id, transformed_inner);
        }

        transformed_state
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawPresenceDiff {
    joins: RawPresenceState,
    leaves: RawPresenceState,
}

impl Into<PresenceDiff> for RawPresenceDiff {
    fn into(self) -> PresenceDiff {
        PresenceDiff {
            joins: self.joins.into(),
            leaves: self.leaves.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PresenceDiff {
    joins: PresenceState,
    leaves: PresenceState,
}

#[derive(Default)]
pub struct RealtimePresence {
    pub state: PresenceState,
    callbacks: HashMap<PresenceEvent, Vec<Box<dyn FnMut(String, PresenceState, PresenceState)>>>,
}

impl RealtimePresence {
    pub fn add_callback(
        &mut self,
        event: PresenceEvent,
        callback: Box<dyn FnMut(String, PresenceState, PresenceState)>,
    ) {
        if let None = self.callbacks.get(&event) {
            self.callbacks.insert(event.clone(), vec![]);
        }

        self.callbacks
            .get_mut(&event)
            .unwrap_or(&mut vec![])
            .push(callback);
    }

    pub fn sync(&mut self, new_state: PresenceState) {
        // TODO state? functional? Nah both mixed together. lol and also lmao even
        let joins: PresenceState = new_state
            .0
            .clone()
            .into_iter()
            .map(|(new_id, mut new_phx_map)| {
                new_phx_map.retain(|new_phx_ref, _new_state_data| {
                    let mut retain = true;
                    let _ = self
                        .state
                        .0
                        .clone()
                        .into_iter()
                        .map(|(_self_id, self_phx_map)| {
                            if self_phx_map.contains_key(new_phx_ref) {
                                retain = false;
                            }
                        });
                    retain
                });

                (new_id, new_phx_map)
            })
            .collect();

        let leaves: PresenceState = self
            .state
            .0
            .clone()
            .into_iter()
            .map(|(current_id, mut current_phx_map)| {
                current_phx_map.retain(|current_phx_ref, _current_state_data| {
                    let mut retain = false;
                    let _ = new_state
                        .0
                        .clone()
                        .into_iter()
                        .map(|(_new_id, new_phx_map)| {
                            if !new_phx_map.contains_key(current_phx_ref) {
                                retain = true;
                            }
                        });
                    retain
                });

                (current_id, current_phx_map)
            })
            .collect();

        let prev_state = self.state.clone();

        self.sync_diff(PresenceDiff { joins, leaves });

        for (id, _data) in self.state.0.clone() {
            for cb in self
                .callbacks
                .get_mut(&PresenceEvent::Sync)
                .unwrap_or(&mut vec![])
            {
                cb.as_mut()(id.clone(), prev_state.clone(), self.state.clone());
            }
        }
    }

    pub fn sync_diff(&mut self, diff: PresenceDiff) -> &PresenceState {
        // mutate own state with diff
        // return new state
        // trigger diff callbacks

        for (id, _data) in diff.joins.0.clone() {
            for cb in self
                .callbacks
                .get_mut(&PresenceEvent::Join)
                .unwrap_or(&mut vec![])
            {
                cb.as_mut()(id.clone(), self.state.clone(), diff.clone().joins);
            }
        }

        for (id, _data) in diff.leaves.0.clone() {
            for cb in self
                .callbacks
                .get_mut(&PresenceEvent::Leave)
                .unwrap_or(&mut vec![])
            {
                cb.as_mut()(id.clone(), self.state.clone(), diff.clone().joins);
            }

            self.state.0.remove(&id);
        }

        self.state.0.extend(diff.joins.0);

        &self.state
    }
}
