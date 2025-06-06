use wasm_bindgen::prelude::*;
//use specta::{Type, ts};
use sha3::{Sha3_256, Digest};
use serde::{Serialize, Deserialize as SerdeDeserialize};
use time::Time;

use uuid::Uuid;
use std::{fmt::Display, ops::Deref};
use spacetimedb::{
    sats::{impl_deserialize, impl_serialize, impl_st},
    ReducerContext,
};

// st_uuid implementation
//#[derive(Debug, Serialize, SerdeDeserialize, Type)]
#[wasm_bindgen]
pub struct StUuid(pub Uuid);

// spacetimedb impls
impl_st!([] StUuid, spacetimedb::sats::AlgebraicType::String);
impl_serialize!([] StUuid, (self, ser) => {
    ser.serialize_str(self.hyphenated().encode_upper(&mut Uuid::encode_buffer()))
});
impl_deserialize!([] StUuid, de => {
    let s = <std::string::String as spacetimedb::Deserialize>::deserialize(de).map(|s| s.into_boxed_str())?;
    Ok(Uuid::parse_str(&s).map(|u| u.into()).expect("Failed to Deserialize to UUID"))
});

impl StUuid {
    pub fn new(ctx: &ReducerContext) -> Self {
        StUuid(Uuid::from_bytes(ctx.random()))
    }
}

impl Display for StUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Deref for StUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for StUuid {
    fn from(val: Uuid) -> Self {
        Self(val)
    }
}

// Seed and related types

//#[derive(Type)]
#[wasm_bindgen]
pub struct Seed {
    pub seed: String,
    pub s: String,
    pub f: String,
    pub r: Rarity,
    pub v: String,
    pub t: Time,
    pub u: StUuid,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum Gender {
    Male,
    Female,
    Other,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Color {
    pub hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum IrisShape {
    Round,
    Almond,
    Heart,
    Dot,
    Star,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Iris {
    pub color: Color,
    pub shape: IrisShape
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Eye {
    pub color: Color,
    pub iris: Iris,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Eyes {
    pub left: Eye,
    pub light: Eye,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct HairColor {
    pub gradient: bool,
    pub color1: Color,
    pub color2: Color,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum HairStyle {
    Variant1,
    Variant2,
    Variant3,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Hair {
    pub color: HairColor,
    pub length: u16,
    pub style: HairStyle,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Physical {
    pub race: Race,
    pub gender: Gender,
    pub age: u16, // in years
    pub height: u16, // in cm
    pub weight: u16, // in kg
    pub eyes: Eyes,
    pub hair: Hair,
    pub skin_color: Color,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Name {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Parents {
    pub mother: Option<Seed>,
    pub father: Option<Seed>,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Siblings {
    pub count: u8,
    pub sibling1: Option<Seed>,
    pub sibling2: Option<Seed>,
    pub sibling3: Option<Seed>,
    pub sibling4: Option<Seed>,
    pub sibling5: Option<Seed>,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Children {
    pub count: u8,
    pub children1: Option<Seed>,
    pub children2: Option<Seed>,
    pub children3: Option<Seed>,
    pub children4: Option<Seed>,
    pub children5: Option<Seed>,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Relationship {
    pub status: MaritalStatus,
    pub parents: Parents,
    pub siblings: Siblings,
    pub children: Children,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum Rarity {
    Common,
    Rare,
    SuperRare,
    SuperSuperRare,
    UltraRare,
    Epic,
    Legendary,
    Mythic,
    Unique,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum Race {
    Human,
}

//#[derive(Type)]
#[wasm_bindgen]
pub enum Class {
    Villager,
}

//#[derive(Type)]
#[wasm_bindgen]
pub struct Metadata {
    pub seed: Seed,
    pub name: Name,
    pub class: Class,
    pub physical: Physical,
    pub relationship: Relationship,
    pub money: u32,
}

#[wasm_bindgen]
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    return result.iter().map(|b| format!("{:02x}", b)).collect();
}