CREATE TABLE guilds (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    mod_id VARCHAR(255) NULL,
    audit_id VARCHAR(255) NULL,
    welcome_id VARCHAR(255) NULL
);

CREATE TYPE infraction_type AS ENUM ('ban', 'kick', 'mute', 'warn');

CREATE TABLE infractions (
    guild_id VARCHAR(255) NOT NULL,
    member_id VARCHAR(255) NOT NULL,
    moderator_id VARCHAR(255) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    infraction_type infraction_type NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);