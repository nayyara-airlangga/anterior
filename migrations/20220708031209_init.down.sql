-- Add down migration script here
DROP TABLE IF EXISTS "osiris"."comments";

DROP TABLE IF EXISTS "osiris"."posts";

DROP TABLE IF EXISTS "osiris"."users";

DROP SCHEMA IF EXISTS "osiris" RESTRICT;
