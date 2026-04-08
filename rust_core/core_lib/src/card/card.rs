use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureData {
    pub attack: u32,
    pub health: u32,
    pub max_health: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponData {
    pub attack: u32,
    pub durability: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactData {
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellData {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardKind {
    Creature(CreatureData),
    Spell(SpellData),
    Artifact(ArtifactData),
    Weapon(WeaponData)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub expansion_id: Uuid,
    pub name: String,
    pub description: String,
    pub cost: Cost,
    pub kind: CardKind
}