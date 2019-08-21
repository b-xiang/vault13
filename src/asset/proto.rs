mod db;
mod id;

use enumflags::BitFlags;
use enum_map::EnumMap;
use std::ops::RangeInclusive;

pub use id::Pid;
pub use db::ProtoDb;

use super::*;
use crate::asset::frame::FrameId;
use crate::asset::EntityKind;
use crate::util::{enum_iter, EnumIter};

#[derive(Debug)]
pub struct Proto {
    pub pid: Pid,
    pub message_id: i32,
    pub fid: FrameId,
    pub light_radius: i32,
    pub light_intensity: i32,
    pub flags: BitFlags<Flag>,
    pub flags_ext: BitFlags<FlagExt>,
    pub script_id: Option<u32>,
    pub proto: Variant,
}

impl Proto {
    pub fn kind(&self) -> ExactEntityKind {
        self.proto.kind()
    }
}

#[derive(Debug)]
pub enum Variant {
    Item(Item),
    Critter(Critter),
    Scenery(Scenery),
    Wall(Wall),
    SqrTile(SqrTile),
    Misc,
}

impl Variant {
    pub fn kind(&self) -> ExactEntityKind {
        use self::Variant::*;
        match self {
            Item(ref v) => ExactEntityKind::Item(v.item.kind()),
            Critter(_) => ExactEntityKind::Critter,
            Scenery(ref v) => ExactEntityKind::Scenery(v.scenery.kind()),
            Wall(_) => ExactEntityKind::Wall,
            SqrTile(_) => ExactEntityKind::SqrTile,
            Misc => ExactEntityKind::Misc,
        }
    }

    pub fn item(&self) -> Option<&Item> {
        if let Variant::Item(ref v) = self { Some(v) } else { None }
    }

    pub fn critter(&self) -> Option<&Critter> {
        if let Variant::Critter(ref v) = self { Some(v) } else { None }
    }

    pub fn scenery(&self) -> Option<&Scenery> {
        if let Variant::Scenery(ref v) = self { Some(v) } else { None }
    }

    pub fn wall(&self) -> Option<&Wall> {
        if let Variant::Wall(ref v) = self { Some(v) } else { None }
    }

    pub fn sqr_tile(&self) -> Option<&SqrTile> {
        if let Variant::SqrTile(ref v) = self { Some(v) } else { None }
    }

    pub fn is_misc(&self) -> bool {
        if let Variant::Misc = self { true } else { false }
    }
}

#[derive(Debug)]
pub struct Item {
    pub material: Material,
    pub size: i32,
    pub weight: i32,
    pub price: i32,
    pub inventory_fid: Option<FrameId>,
    pub sound_id: u8,
    pub item: ItemVariant,
}

#[derive(Debug)]
pub enum ItemVariant {
    Armor(Armor),
    Container(Container),
    Drug(Drug),
    Weapon(Weapon),
    Ammo(Ammo),
    Misc(MiscItem),
    Key(Key),
}

impl ItemVariant {
    pub fn kind(&self) -> ItemKind {
        use self::ItemVariant::*;
        match self {
            Armor(_)        => ItemKind::Armor,
            Container(_)    => ItemKind::Container,
            Drug(_)         => ItemKind::Drug,
            Weapon(_)       => ItemKind::Weapon,
            Ammo(_)         => ItemKind::Ammo,
            Misc(_)         => ItemKind::Misc,
            Key(_)          => ItemKind::Key,
        }
    }
}

#[derive(Debug)]
pub struct Armor {
  pub armor_class: i32,
  pub damage_resistance: EnumMap<DamageKind, i32>,
  pub damage_threshold: EnumMap<DamageKind, i32>,
  pub perk: Option<Perk>,
  pub male_fid: FrameId,
  pub female_fid: FrameId,
}

#[derive(Debug)]
pub struct Container {
    pub capacity: i32,
    pub flags: BitFlags<ContainerFlag>,
}

#[derive(Clone, Copy, Debug, EnumFlags, Eq, PartialEq)]
#[repr(u32)]
pub enum ContainerFlag {
    CannotPickUp = 1,
    MagicHandsGround = 8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrugEffectModifier {
    Fixed(i32),
    Random(i32, i32),
}

#[derive(Debug)]
pub struct DrugEffect {
    pub delay: u32,
    pub stat: Stat,
    pub modifier: DrugEffectModifier,
}

#[derive(Debug)]
pub struct DrugAddiction {
    pub chance: u32,
    pub perk: Option<Perk>,
    pub delay: u32,
}

#[derive(Debug)]
pub struct Drug {
    pub effects: Vec<DrugEffect>,
    pub addiction: DrugAddiction,
}

#[derive(Clone, Debug)]
pub struct Dual<T> {
    pub primary: T,
    pub secondary: T,
}

#[derive(Debug)]
pub struct Weapon {
    pub attack_kind: Dual<AttackKind>,
    pub animation_code: WeaponKind,
    pub damage: RangeInclusive<i32>,
    pub damage_kind: DamageKind,
    pub max_range: Dual<i32>,
    pub projectile_pid: Option<Pid>,
    pub min_strength: i32,
    pub ap_cost: Dual<i32>,
    pub crit_failure_table: i32,
    pub perk: Option<Perk>,
    // Number of bullets per burst shot.
    pub burst_bullet_count: i32,
    // proto.msg:300
    pub caliber: i32,
    pub ammo_pid: Option<Pid>,
    /// Magazine capacity.
    pub max_ammo: i32,
    pub sound_id: u8,
}

#[derive(Debug)]
pub struct Ammo {
    pub caliber: i32,
    pub magazine_size: i32,
    pub ac_modifier: i32,
    pub dr_modifier: i32,
    pub damage_mult: i32,
    pub damage_div: i32,
}

#[derive(Debug)]
pub struct MiscItem {
    pub charge_pid: Option<Pid>,
    pub charge_kind: u32,
    pub max_charges: i32,
}

#[derive(Debug)]
pub struct Key {
    pub id: i32,
}

#[derive(Debug)]
pub struct Critter {
    pub flags: BitFlags<CritterFlag>,
    pub base_stats: EnumMap<Stat, i32>,
    pub bonus_stats: EnumMap<Stat, i32>,
    pub skills: EnumMap<Skill, i32>,
    //proto.msg:400
    //0x0 - biped (двуногие)
    //0x1 - quadruped (четвероногие)
    //0x2 - robotic (роботы)
    pub body_kind: u32,
    pub experience: i32,
    //proto.msg:1450
    pub kill_kind: CritterKillKind,
    pub damage_kind: DamageKind,
    pub head_fid: Option<FrameId>,
    pub ai_packet: u32,
    pub team_id: u32,
}

#[derive(Clone, Copy, Debug, EnumFlags, Eq, PartialEq)]
#[repr(u32)]
pub enum CritterFlag {
    NoBarter        = 0x00000002, // Can barter with.
    NoSteal         = 0x00000020, // Can't steal from.
    NoDrop          = 0x00000040, // Doesn't drop items.
    NoLoseLimbs     = 0x00000080, // Can't shoot off limbs.
    Ages            = 0x00000100, // Dead body doesn't disappear.
    NoHeal          = 0x00000200, // HP doesn't restore over time.
    Invulnerable    = 0x00000400,
    NoDeadBody      = 0x00000800, // Dead body disappears immediately.
    SpecialDeath    = 0x00001000, // Has special death animation.
    RangedMelee     = 0x00002000, // Melee attack is possible at a distance.
    NoKnock         = 0x00004000, // Can't knock down.
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Primitive)]
pub enum CritterKillKind {
  Man = 0x0,
  Woman = 0x1,
  Children = 0x2,
  SuperMutant = 0x3,
  Ghoul = 0x4,
  Brahmin = 0x5,
  Radscorpion = 0x6,
  Rat = 0x7,
  Floater = 0x8,
  Centaur = 0x9,
  Robot = 0xA,
  Dog = 0xB,
  Manti = 0xC,
  DeathClaw = 0xD,
  Plant = 0xE,
  Gecko = 0xF,
  Alien = 0x10,
  GiantAnt = 0x11,
  BigBadBoss = 0x12,
}

#[derive(Debug)]
pub struct Scenery {
    pub material: Material,
    pub sound_id: u8,
    pub scenery: SceneryVariant,
}

#[derive(Debug)]
pub enum SceneryVariant {
    Door(Door),
    Stairs(Stairs),
    Elevator(Elevator),
    Ladder(Ladder),
    Misc,
}

impl SceneryVariant {
    pub fn kind(&self) -> SceneryKind {
        use self::SceneryVariant::*;
        match self {
            Door(_) => SceneryKind::Door,
            Stairs(_) => SceneryKind::Stairs,
            Elevator(_) => SceneryKind::Elevator,
            Ladder(ref l) => match l.kind {
                LadderKind::Up => SceneryKind::LadderUp,
                LadderKind::Down => SceneryKind::LadderDown,
            }
            Misc => SceneryKind::Misc,
        }
    }
}

#[derive(Debug)]
pub struct Door {
    pub flags: u32,
    pub key_id: u32,
}

#[derive(Debug)]
pub struct Stairs {
    pub elevation_and_tile: u32,
    pub map_id: u32,
}

#[derive(Debug)]
pub struct Elevator {
    pub kind: u32,
    pub level: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LadderKind {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Ladder {
    pub kind: LadderKind,
    pub elevation_and_tile: u32,
}

#[derive(Debug)]
pub struct Wall {
    pub material: Material,
}

#[derive(Debug)]
pub struct SqrTile {
    pub material: Material,
}

// Subset that has prototypes.
pub fn proto_entity_kinds() -> EnumIter<EntityKind> {
    enum_iter(..=EntityKind::Misc)
}