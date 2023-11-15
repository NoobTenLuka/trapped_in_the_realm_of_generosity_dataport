use rust_decimal::Decimal;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Datacenter {
    id: i16,
    name: String,
    region: String,
}

#[derive(sqlx::FromRow)]
pub struct GameServer {
    id: i16,
    name: String,
    datacenter_id: i16,
}

#[derive(sqlx::FromRow)]
pub struct Area {
    id: i16,
    name: String,
    description: String,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "location")]
pub struct Location {
    /* As per database definition, only decimal numbers between
    * Min Coordinate = 0000.000
    * and
    * Max Coordinate = 9999.999
    * will be accepted into the database
    */
    x_coordinate: Decimal,
    y_coordinate: Decimal,
    z_coordinate: Decimal,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "instance_type")]
pub enum InstanceType { Dungeon, Trial, Raid }

#[derive(sqlx::FromRow)]
pub struct Instance {
    id: Uuid,
    name: String,
    description: String,
    #[sqlx(rename = "type")]
    instance_type: InstanceType,
    player_count: i16,
    min_level: i16,
}

#[derive(sqlx::FromRow)]
pub struct Character {
    id: Uuid,
    name: String,
    digidollar: i32,
    game_server_id: i16,
    area_id: i16,
    location: Location,
    instance_id: Option<Uuid>,
    instance_location: Option<Location>,
    keycloak_user_id: Uuid,
    creation_date: OffsetDateTime,
    playtime: i64,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "class_type")]
pub enum ClassType { Melee, Ranged, Caster, Tank, Healer }

#[derive(sqlx::FromRow)]
pub struct Class {
    id: i16,
    name: String,
    description: String,
    #[sqlx(rename = "type")]
    class_type: ClassType,
}

#[derive(sqlx::FromRow)]
pub struct CharacterClass {
    character_id: Uuid,
    class_id: i16,
    level: i16,
    experience: i32,
}

#[derive(sqlx::FromRow)]
pub struct Item {
    id: Uuid,
    name: String,
    description: String,
    sellable: bool,
    is_key_item: bool,
}

#[derive(sqlx::FromRow)]
pub struct InventoryItem {
    character_id: Uuid,
    item_id: Uuid,
    amount: i16,
    slot: i16,
}

#[derive(sqlx::FromRow)]
pub struct Stat {
    id: i16,
    name: String,
}

#[derive(sqlx::FromRow)]
pub struct ItemStat {
    item_id: Uuid,
    stat_id: i16,
    value: i32,
}

#[derive(sqlx::FromRow)]
pub struct NPC {
    id: Uuid,
    area_id: i16,
    location: Location,
    name: String,
    default_conversation: String,
    visible_without_quest: bool,
}

#[derive(sqlx::FromRow)]
pub struct ShopType {
    id: i16,
    name: String,
}

#[derive(sqlx::FromRow)]
pub struct Shop {
    id: Uuid,
    seller: Uuid,
    description: String,
    #[sqlx(rename = "type")]
    shop_type: i16,
}

#[derive(sqlx::FromRow)]
pub struct ShopSellsItem {
    id: i16,
    shop_id: Uuid,
    item_id: Uuid,
    price: Option<i32>,
    default_order: i16,
}

#[derive(sqlx::FromRow)]
pub struct ItemAsPrice {
    shop_sells_item_id: i16,
    item_id: Uuid,
    amount: i16,
}

#[derive(sqlx::FromRow)]
pub struct Enemy {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(sqlx::FromRow)]
pub struct EnemyDrop {
    enemy_id: Uuid,
    item_id: Uuid,
    min_roll: i16,
    min: i16,
    max: i16,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "quest_type")]
pub enum QuestType { Normal, Unlocking, MainStory }

#[derive(sqlx::FromRow)]
pub struct Quest {
    id: Uuid,
    level_requirement: i16,
    class_requirement: Option<i16>,
    class_type_requirement: ClassType,
    requirement_disjunction: bool,
    #[sqlx(rename = "type")]
    quest_type: QuestType,
    digidollar_reward: Option<i32>,
    experience_reward: Option<i32>,
    unlocks_class: Option<i16>,
}

#[derive(sqlx::FromRow)]
pub struct QuestRequirement {
    requirement: Uuid,
    unlocks: Uuid,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "quest_state")]
pub enum QuestState { Unlocked, Accepted, Cleared }

#[derive(sqlx::FromRow)]
pub struct QuestCharacterRelation {
    character_id: Uuid,
    quest_id: Uuid,
    progression: i16,
    sub_progression: Option<i16>,
    state: QuestState,
}

#[derive(sqlx::FromRow)]
pub struct QuestItemReward {
    quest_id: Uuid,
    item_id: Uuid,
    optional: bool,
    amount: i16,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "instance_state")]
pub enum InstanceState { Unlocked, Cleared }

#[derive(sqlx::FromRow)]
pub struct CharacterInstanceRelation {
    character_id: Uuid,
    instance_id: Uuid,
    state: InstanceState,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "objective_item_type")]
pub enum ObjectiveItemType { GiveItem, RemoveItem }

#[derive(sqlx::FromRow)]
pub struct QuestObjective {
    id: Uuid,
    description: String,
    dialogue: Option<String>,
    amount: Option<i16>,
    order_in_quest: i16,
    npc: Option<Uuid>,
    item: Option<Uuid>,
    item_type: Option<ObjectiveItemType>,
    enemy: Option<Enemy>,
    instance: Option<Instance>,
}