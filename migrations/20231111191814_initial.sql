CREATE TABLE datacenter
(
    id     SMALLSERIAL PRIMARY KEY,
    name   TEXT NOT NULL,
    region TEXT NOT NULL
);

CREATE TABLE game_server
(
    id            SMALLSERIAL PRIMARY KEY,
    name          TEXT     NOT NULL,
    datacenter_id SMALLINT NOT NULL REFERENCES datacenter
);

CREATE TABLE area
(
    id          SMALLSERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TYPE location AS
(
    -- Min Coordinate = 0000.000
    -- Max Coordinate = 9999.999
    x_coordinate DECIMAL(7, 3),
    y_coordinate DECIMAL(7, 3),
    z_coordinate DECIMAL(7, 3)
);

CREATE TABLE character
(
    id                uuid PRIMARY KEY  DEFAULT gen_random_uuid(),
    name              TEXT     NOT NULL,
    digidollar        INT      NOT NULL DEFAULT 0,
    game_server_id    SMALLINT REFERENCES game_server,
    area_id           SMALLINT NOT NULL,
    location          location NOT NULL,
    instance_id       uuid REFERENCES instance,
    instance_location location          DEFAULT NULL,
    keycloak_user_id  uuid     NOT NULL,
    creation_date     timestamptz       DEFAULT now(),
    -- Playtime in ms
    playtime          BIGINT   NOT NULL DEFAULT 0
);

CREATE INDEX character_keycloak_uid ON character (keycloak_user_id);

CREATE TYPE class_type AS ENUM ('Melee', 'Ranged', 'Caster', 'Tank', 'Healer');

CREATE TABLE class
(
    id          SMALLSERIAL PRIMARY KEY,
    name        TEXT       NOT NULL,
    description TEXT       NOT NULL,
    type        class_type NOT NULL
);

CREATE TABLE character_class
(
    character_id uuid REFERENCES character,
    class_id     SMALLINT REFERENCES class,
    level        SMALLINT NOT NULL DEFAULT 1,
    experience   INT      NOT NULL DEFAULT 0,
    PRIMARY KEY (character_id, class_id)
);

CREATE TABLE item
(
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    description TEXT    NOT NULL,
    sellable    BOOLEAN NOT NULL,
    is_key_item BOOLEAN NOT NULL
);

CREATE TABLE inventory_item
(
    character_id uuid REFERENCES character,
    item_id      uuid REFERENCES item,
    amount       SMALLINT NOT NULL,
    slot         SMALLINT NOT NULL,
    PRIMARY KEY (character_id, item_id)
);

CREATE TABLE stat
(
    id   SMALLSERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE item_stat
(
    item_id uuid REFERENCES item,
    stat_id SMALLINT REFERENCES stat,
    value   INT NOT NULL,
    PRIMARY KEY (item_id, stat_id)
);

CREATE TABLE npc
(
    id                    uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    area_id               SMALLINT REFERENCES area,
    location              location NOT NULL,
    name                  TEXT     NOT NULL,
    default_conversation  TEXT     NOT NULL,
    visible_without_quest BOOLEAN          DEFAULT true
);

CREATE TABLE shop_type
(
    id   SMALLSERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE shop
(
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    seller      uuid REFERENCES npc NOT NULL UNIQUE,
    description TEXT                NOT NULL,
    type        uuid REFERENCES shop_type
);

CREATE TABLE shop_sells_item
(
    id            SMALLSERIAL PRIMARY KEY,
    shop_id       uuid REFERENCES shop,
    item_id       uuid REFERENCES item,
    price         INT,
    default_order SMALLINT NOT NULL
);

CREATE TABLE item_as_price
(
    shop_sells_item_id SMALLINT REFERENCES shop_sells_item,
    item_id            uuid REFERENCES item,
    amount             SMALLINT NOT NULL,
    PRIMARY KEY (shop_sells_item_id, item_id)
);

CREATE TABLE enemy
(
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE enemy_drop
(
    enemy_id uuid REFERENCES enemy,
    item_id  uuid REFERENCES item,
    -- \frac{x}{2}^{2}+20 aka ( x / 2 ) ^ 2 + min_roll. Where x is the amount the player already has of this item
    -- and y is the roll needed to get the item
    -- The minimum roll to get this item when the player does not have it already. Players roll from 1 - 100.
    min_roll SMALLINT NOT NULL,
    min      SMALLINT NOT NULL,
    max      SMALLINT NOT NULL,
    PRIMARY KEY (enemy_id, item_id)
);

CREATE TYPE quest_type AS ENUM ('Normal', 'Unlocking', 'MainStory');

CREATE TABLE quest
(
    id                      uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    level_requirement       SMALLINT   NOT NULL,
    class_requirement       uuid REFERENCES class,
    class_type_requirement  class_type,
    requirement_disjunction BOOLEAN    NOT NULL,
    type                    quest_type NOT NULL,
    digidollar_reward       INT,
    experience_reward       INT,
    unlocks_class           uuid REFERENCES class
);

CREATE TABLE quest_requirement
(
    requirement uuid REFERENCES quest,
    unlocks     uuid REFERENCES quest,
    PRIMARY KEY (requirement, unlocks)
);

CREATE TYPE quest_state AS ENUM ('Unlocked', 'Accepted', 'Cleared');

CREATE TABLE quest_character_relation
(
    character_id    uuid REFERENCES character,
    quest_id        uuid REFERENCES quest,
    progression     SMALLINT    NOT NULL DEFAULT 0,
    sub_progression SMALLINT,
    state           quest_state NOT NULL DEFAULT 'Unlocked',
    PRIMARY KEY (character_id, quest_id)
);

CREATE TABLE quest_item_reward
(
    quest_id uuid REFERENCES quest,
    item_id  uuid REFERENCES item,
    optional BOOLEAN NOT NULL,
    amount   SMALLINT,
    PRIMARY KEY (quest_id, item_id)
);

CREATE TYPE instance_type AS ENUM ('Dungeon', 'Trial', 'Raid');

CREATE TABLE instance
(
    id           uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name         TEXT          NOT NULL,
    description  TEXT          NOT NULL,
    type         instance_type NOT NULL,
    player_count SMALLINT      NOT NULL,
    min_level    SMALLINT      NOT NULL
);

CREATE TYPE instance_state AS ENUM ('Unlocked', 'Completed');

CREATE TABLE character_instance_relation
(
    character_id uuid REFERENCES character,
    instance_id  uuid REFERENCES instance,
    state        instance_state NOT NULL DEFAULT 'Unlocked',
    PRIMARY KEY (character_id, instance_id)
);

CREATE TYPE objective_item_type AS ENUM ('GiveItem', 'RemoveItem');

CREATE TABLE quest_objective
(
    id             uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    description    TEXT                NOT NULL,
    dialogue       TEXT,
    amount         SMALLINT,
    order_in_quest SMALLINT            NOT NULL,
    npc            uuid REFERENCES npc,
    item           uuid REFERENCES item,
    item_type      objective_item_type NOT NULL,
    enemy          uuid REFERENCES enemy,
    instance       uuid REFERENCES instance
);
