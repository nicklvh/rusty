CREATE TABLE IF NOT EXISTS guilds (
    id VARCHAR(255) PRIMARY KEY,
    mod_id VARCHAR(255) NULL,
    audit_id VARCHAR(255) NULL,
    welcome_id VARCHAR(255) NULL
);

CREATE TYPE IF NOT EXISTS infraction_type AS ENUM ('ban', 'kick', 'mute', 'warn');

CREATE TABLE IF NOT EXISTS infractions (
    guild_id VARCHAR(255),
    member_id VARCHAR(255),
    moderator_id VARCHAR(255),
    reason VARCHAR(255),
    infraction_type infraction_type,
);