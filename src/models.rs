use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Datacenter {
    pub id: i16,
    pub name: String,
    pub region: String,
}

#[derive(sqlx::FromRow)]
pub struct GameServer {
    pub id: i16,
    pub name: String,
    pub datacenter_id: i16,
}

#[derive(sqlx::FromRow)]
pub struct Area {
    pub id: i16,
    pub name: String,
    pub description: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, Copy, Clone)]
#[sqlx(type_name = "location")]
pub struct Location {
    /* As per database definition, only decimal numbers between
    * Min Coordinate = 0000.000
    * and
    * Max Coordinate = 9999.999
    * will be accepted into the database
    */
    pub x_coordinate: Decimal,
    pub y_coordinate: Decimal,
    pub z_coordinate: Decimal,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "instance_type")]
pub enum InstanceType { Dungeon, Trial, Raid }

#[derive(sqlx::FromRow)]
pub struct Instance {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[sqlx(rename = "type")]
    pub instance_type: InstanceType,
    pub player_count: i16,
    pub min_level: i16,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub digidollar: i32,
    pub game_server_id: i16,
    pub area_id: i16,
    pub location: Location,
    pub instance_id: Option<Uuid>,
    pub instance_location: Option<Location>,
    pub keycloak_user_id: Uuid,
    pub creation_date: OffsetDateTime,
    pub playtime: i64,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "class_type")]
pub enum ClassType { Melee, Ranged, Caster, Tank, Healer }

#[derive(sqlx::FromRow)]
pub struct Class {
    pub id: i16,
    pub name: String,
    pub description: String,
    #[sqlx(rename = "type")]
    pub class_type: ClassType,
}

#[derive(sqlx::FromRow)]
pub struct CharacterClass {
    pub character_id: Uuid,
    pub class_id: i16,
    pub level: i16,
    pub experience: i32,
}

#[derive(sqlx::FromRow)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub sellable: bool,
    pub is_key_item: bool,
}

#[derive(sqlx::FromRow)]
pub struct InventoryItem {
    pub character_id: Uuid,
    pub item_id: Uuid,
    pub amount: i16,
    pub slot: i16,
}

#[derive(sqlx::FromRow)]
pub struct Stat {
    pub id: i16,
    pub name: String,
}

#[derive(sqlx::FromRow)]
pub struct ItemStat {
    pub item_id: Uuid,
    pub stat_id: i16,
    pub value: i32,
}

#[derive(sqlx::FromRow)]
pub struct NPC {
    pub id: Uuid,
    pub area_id: i16,
    pub location: Location,
    pub name: String,
    pub default_conversation: String,
    pub visible_without_quest: bool,
}

#[derive(sqlx::FromRow)]
pub struct ShopType {
    pub id: i16,
    pub name: String,
}

#[derive(sqlx::FromRow)]
pub struct Shop {
    pub id: Uuid,
    pub seller: Uuid,
    pub description: String,
    #[sqlx(rename = "type")]
    pub shop_type: i16,
}

#[derive(sqlx::FromRow)]
pub struct ShopSellsItem {
    pub id: i16,
    pub shop_id: Uuid,
    pub item_id: Uuid,
    pub price: Option<i32>,
    pub default_order: i16,
}

#[derive(sqlx::FromRow)]
pub struct ItemAsPrice {
    pub shop_sells_item_id: i16,
    pub item_id: Uuid,
    pub amount: i16,
}

#[derive(sqlx::FromRow)]
pub struct Enemy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(sqlx::FromRow)]
pub struct EnemyDrop {
    pub enemy_id: Uuid,
    pub item_id: Uuid,
    pub min_roll: i16,
    pub min: i16,
    pub max: i16,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "quest_type")]
pub enum QuestType { Normal, Unlocking, MainStory }

#[derive(sqlx::FromRow)]
pub struct Quest {
    pub id: Uuid,
    pub level_requirement: i16,
    pub class_requirement: Option<i16>,
    pub class_type_requirement: ClassType,
    pub requirement_disjunction: bool,
    #[sqlx(rename = "type")]
    pub quest_type: QuestType,
    pub digidollar_reward: Option<i32>,
    pub experience_reward: Option<i32>,
    pub unlocks_class: Option<i16>,
}

#[derive(sqlx::FromRow)]
pub struct QuestRequirement {
    pub requirement: Uuid,
    pub unlocks: Uuid,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "quest_state")]
pub enum QuestState { Unlocked, Accepted, Cleared }

#[derive(sqlx::FromRow)]
pub struct QuestCharacterRelation {
    pub character_id: Uuid,
    pub quest_id: Uuid,
    pub progression: i16,
    pub sub_progression: Option<i16>,
    pub state: QuestState,
}

#[derive(sqlx::FromRow)]
pub struct QuestItemReward {
    pub quest_id: Uuid,
    pub item_id: Uuid,
    pub optional: bool,
    pub amount: i16,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "instance_state")]
pub enum InstanceState { Unlocked, Cleared }

#[derive(sqlx::FromRow)]
pub struct CharacterInstanceRelation {
    pub character_id: Uuid,
    pub instance_id: Uuid,
    pub state: InstanceState,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "objective_item_type")]
pub enum ObjectiveItemType { GiveItem, RemoveItem }

#[derive(sqlx::FromRow)]
pub struct QuestObjective {
    pub id: Uuid,
    pub description: String,
    pub dialogue: Option<String>,
    pub amount: Option<i16>,
    pub order_in_quest: i16,
    pub npc: Option<Uuid>,
    pub item: Option<Uuid>,
    pub item_type: Option<ObjectiveItemType>,
    pub enemy: Option<Enemy>,
    pub instance: Option<Instance>,
}