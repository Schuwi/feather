use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::AreaEffectCloud;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(AreaEffectCloud)
        .add(EntityKind::AreaEffectCloud);
}
