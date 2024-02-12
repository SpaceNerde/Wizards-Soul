use std::marker;

use bevy::prelude::*;

// credit to https://github.com/stratecide

// TODO: Finetune and abstract stuff away!

// Implement Spell Logic

// Spell Couldown
#[derive(Component)]
pub struct SpellCooldown<S: Spell> {
    pub cooldown: f32,
    _marker: marker::PhantomData<S>,
}

// Spell Bundle
#[derive(Bundle)]
pub struct SpellBundle<S: Spell> {
    spell: S,
    cooldown: SpellCooldown<S>,
}

// Spell Component
// TODO: Add assets!
pub trait Spell: Component {
    fn max_cooldown(&self) -> f32;
    fn cast(&self, commands: &mut Commands, spell_transform: &Transform);
}

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
    }
}

// Basic fire spell
// can ignite enemy 

#[derive(Component)]
pub struct FireSpell {
    pub casts: usize,
    cooldown: f32,
}

impl FireSpell {
    pub fn new(casts: usize, cooldown: f32) -> SpellBundle<Self> {
        SpellBundle {
            spell: Self {
                casts,
                cooldown,
            },
            cooldown: SpellCooldown {
                cooldown,
                _marker: marker::PhantomData,
            }
        }
    }
}

// TODO
impl Spell for FireSpell {
    fn max_cooldown(&self) -> f32 {
        // Placeholder
        1.
    }
    fn cast(&self, commands: &mut Commands, spell_transform: &Transform) {
        // TODO
    }
}
