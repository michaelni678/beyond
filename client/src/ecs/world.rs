use crate::ecs::{actives::Actives, error::EcsError};
use hecs::{DynamicBundle, Entity, Query, QueryBorrow, QueryMut, QueryOne, QueryOneError};

/// Manages entities and their components.
#[derive(Default)]
pub struct World {
  inner: hecs::World,
  pub actives: Actives,
}

impl World {
  /// Create a new world.
  pub fn new() -> Self {
    Self::default()
  }
  /// Spawn an entity.
  pub fn spawn_entity(&mut self, components: impl DynamicBundle) -> Entity {
    self.inner.spawn(components)
  }
  /// Query dynamically.
  #[inline]
  pub fn dynamic_query<Q: Query>(&self) -> QueryBorrow<'_, Q> {
    self.inner.query::<Q>()
  }
  /// Query with a mutable world. This is faster than
  /// `World::dynamic_query`.
  #[inline]
  pub fn standard_query<Q: Query>(&mut self) -> QueryMut<'_, Q> {
    self.inner.query_mut::<Q>()
  }
  /// Inspect a specific entity dynamically.
  #[inline]
  pub fn dynamic_inspect<Q: Query>(&self, entity: Entity) -> Result<QueryOne<'_, Q>, EcsError> {
    self
      .inner
      .query_one::<Q>(entity)
      .map_err(|_| EcsError::EntityNotFound(entity))
  }
  /// Inspect a specific entity with a mutable world. This is
  /// faster than `World::dynamic_inspect`.
  #[inline]
  pub fn standard_inspect<Q: Query>(&mut self, entity: Entity) -> Result<Q::Item<'_>, EcsError> {
    self
      .inner
      .query_one_mut::<Q>(entity)
      .map_err(|error| match error {
        QueryOneError::NoSuchEntity => EcsError::EntityNotFound(entity),
        QueryOneError::Unsatisfied => EcsError::UnsatisfiedInspect,
      })
  }
}
