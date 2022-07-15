-- Add down migration script here
DROP TABLE IF EXISTS "posterior"."comments";

DROP TABLE IF EXISTS "posterior"."posts";

DROP TABLE IF EXISTS "posterior"."users";

DROP SCHEMA IF EXISTS "posterior" RESTRICT;
