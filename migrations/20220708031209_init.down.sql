-- Add down migration script here
DROP TABLE "posterior"."comments";

DROP TABLE "posterior"."posts";

DROP TABLE "posterior"."users";

DROP SCHEMA IF EXISTS "posterior" RESTRICT;
